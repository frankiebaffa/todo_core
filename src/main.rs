use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string as to_json_string;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FormatError;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
const NEW_CONTENT: &'static str = include_str!("../docs/new.md");
#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    // Flags
    /// Creates a new list-item from item-text
    #[clap(short, long)]
    add: bool,
    /// Checks off a list-item by item-number
    #[clap(short, long)]
    check: bool,
    /// Uses the list-name defined in env-var TODO_LIST when a name is not passed
    #[clap(short, long)]
    default_list: bool,
    /// Creates a new list
    #[clap(short, long)]
    new: bool,
    /// Displays list by name
    #[clap(short, long)]
    show: bool,
    /// Silences all messages (overrides verbose flag)
    #[clap(short, long)]
    quiet: bool,
    /// Unchecks a list-item by item-number
    #[clap(short, long)]
    uncheck: bool,
    /// Prints verbose messages during output
    #[clap(short, long)]
    verbose: bool,
    // Options
    /// Selects an item within a list by number
    #[clap(short, long)]
    item: Option<i32>,
    /// Selects a list by path (w/o file extension)
    #[clap(short, long)]
    list_path: Option<String>,
    /// Adds an item to a list by message text
    #[clap(short, long)]
    message: Option<String>,
    #[clap(default_value_t=String::new())]
    buffer: String,
}
impl Args {
    fn check_env(&mut self) {
        match self.list_path {
            Some(_) => {},
            None => {
                match std::env::var("TODO_LIST") {
                    Ok(v) => self.list_path = Some(v),
                    Err(_) => {},
                }
            },
        }
    }
    fn print(&mut self, msg: impl AsRef<str>) {
        self.buffer.push_str(&format!("\n{}", msg.as_ref()));
    }
    fn q_print(&mut self, msg: impl AsRef<str>) {
        if !self.quiet {
            self.print(msg);
        }
    }
    fn v_print(&mut self, msg: impl AsRef<str>) {
        if self.verbose {
            self.q_print(msg);
        }
    }
    fn c_exit(&mut self, code: ExitCode) -> ! {
        self.v_print(format!("{}", code));
        if !self.buffer.is_empty() {
            println!("{}", self.buffer);
        }
        exit(code.into());
    }
}
enum ExitCode<'exit> {
    Success,
    NoListName,
    NoListItemMessage(&'exit mut PathBuf),
    NoListItemNumber(&'exit mut PathBuf),
    FileExists(&'exit mut PathBuf),
    FileDoesNotExist(&'exit mut PathBuf),
    //PathFailed(&'exit mut PathBuf),
    //FileCreationFailed(&'exit mut PathBuf),
    FailedToWrite(&'exit mut PathBuf),
    FailedToRead(&'exit mut PathBuf),
    FailedToOpen(&'exit mut PathBuf),
}
impl<'exit> Into<i32> for ExitCode<'exit> {
    fn into(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::NoListName => 3,
            Self::NoListItemMessage(_) => 4,
            Self::NoListItemNumber(_) => 5,
            Self::FileExists(_) => 6,
            Self::FileDoesNotExist(_) => 7,
            //Self::PathFailed(_) => 8,
            //Self::FileCreationFailed(_) => 9,
            Self::FailedToWrite(_) => 10,
            Self::FailedToRead(_) => 11,
            Self::FailedToOpen(_) => 12,
        }
    }
}
impl<'exit> Display for ExitCode<'exit> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match self {
            Self::Success => write!(f, "Success"),
            Self::NoListName => f.write_str("No list name for list"),
            Self::NoListItemMessage(s) => {
                return f.write_str(&format!("No item-message for list \"{}\"", s.to_str().unwrap()));
            },
            Self::NoListItemNumber(s) => {
                return f.write_str(&format!("No item-number for list \"{}\"", s.to_str().unwrap()));
            },
            Self::FileExists(s) => {
                return f.write_str(&format!("File exists at path \"{}\"", s.to_str().unwrap()));
            },
            Self::FileDoesNotExist(s) => {
                return f.write_str(&format!("File does not exist at path \"{}\"", s.to_str().unwrap()));
            },
            //Self::PathFailed(s) => {
            //    return f.write_str(&format!("Failed to derive path from \"{}\"", s.to_str().unwrap()));
            //},
            //Self::FileCreationFailed(s) => {
            //    return f.write_str(&format!("Failed to create file at \"{}\"", s.to_str().unwrap()));
            //},
            Self::FailedToWrite(s) => {
                return f.write_str(&format!("Failed to write to file \"{}\"", s.to_str().unwrap()));
            },
            Self::FailedToRead(s) => {
                return f.write_str(&format!("Failed to read file \"{}\"", s.to_str().unwrap()));
            },
            Self::FailedToOpen(s) => {
                return f.write_str(&format!("Failed to open file \"{}\"", s.to_str().unwrap()));
            },
        }
    }
}
enum PathExitCondition {
    Exists,
    NotExists,
    Ignore,
}
#[derive(Serialize, Deserialize)]
struct Item {
    number: i32,
    checked: bool,
    text: String,
    sub_items: Vec<Item>,
    created: DateTime<Local>,
    last_updated: DateTime<Local>,
}
#[derive(Serialize, Deserialize)]
struct List {
    name: String,
    items: Vec<Item>,
    created: DateTime<Local>,
    last_updated: DateTime<Local>,
}
impl List {
    fn new(name: String) -> List {
        Self {
            name,
            items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
        }
    }
}
struct Container {
    file: File,
    list: List,
}
impl Container {
    fn create(ctx: &Args, path: &mut PathBuf) -> Container {
        if path.exists() {
            ctx.c_exit(ExitCode::FileExists(path));
        }
        let file = match File::create(path) {
            Ok(file) => file,
            Err(_) => {
                ctx.c_exit(ExitCode::FailedToOpen(path));
            },
        };
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let list = List::new(name);
        Self {
            file,
            list,
        }
    }
    fn load(ctx: &Args, path: &mut PathBuf) -> Container {
        if !path.exists() {
            ctx.c_exit(ExitCode::FileExists(path));
        }
        let mut file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(_) => ctx.c_exit(ExitCode::FailedToOpen(path)),
        };
    }
}
fn get_path(ctx: &mut Args, path: &mut PathBuf, chk: PathExitCondition) {
    if ctx.list_path.is_some() {
        let list = ctx.list_path.clone().unwrap();
        path.push(format!("{}.md", &list));
        match chk {
            PathExitCondition::Exists => {
                if path.exists() {
                    ctx.c_exit(ExitCode::FileExists(path));
                }
            },
            PathExitCondition::NotExists => {
                if !path.exists() {
                    ctx.v_print(
                        format!(
                            "File at \"{}\" does not exist",
                            &path.to_str().unwrap()
                        )
                    );
                    ctx.c_exit(ExitCode::FileDoesNotExist(path));
                }
            },
            PathExitCondition::Ignore => {},
        }
    } else {
        ctx.c_exit(ExitCode::NoListName);
    }
}
fn get_next_item_number(content: impl AsRef<str>) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            "([0-9]+)\\.\\s\\[(\\s|x)\\]"
        ).unwrap();
    }
    let mut nums = Vec::new();
    for found in RE.captures_iter(content.as_ref()) {
        let num_str = found.get(1).unwrap().as_str();
        let num = num_str.parse::<i32>().unwrap();
        nums.push(num);
    }
    nums.sort();
    return nums.last().unwrap_or(&0).clone() + 1;
}
fn update_last_edit(content: impl AsRef<str>, now: String) -> String {
    let c = content.as_ref();
    let start = c.find("Last Edit :").unwrap();
    let subst = &c[start..];
    let end = subst.find("\n").unwrap();
    let date_line = &subst[0..end];
    return c.replace(&date_line, &format!("Last Edit : {}", now)).clone();
}
fn get_now() -> String {
    return Local::now().to_rfc3339();
}
fn get_content_from_file(ctx: &mut Args, path: &mut PathBuf) -> String {
    let mut content = String::new();
    {
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => ctx.c_exit(ExitCode::FailedToOpen(path)),
        };
        match file.read_to_string(&mut content) {
            Ok(_) => {},
            Err(_) => ctx.c_exit(ExitCode::FailedToRead(path)),
        }
    }
    return content;
}
fn overwrite_content(ctx: &mut Args, path: &mut PathBuf, content: String) {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
    {
        Ok(f) => f,
        Err(_) => ctx.c_exit(ExitCode::FailedToOpen(path)),
    };
    match file.write_all(content.as_bytes()) {
        Ok(_) => {},
        Err(_) => ctx.c_exit(ExitCode::FailedToWrite(path)),
    }
}
fn check_item(item: i32, content: impl AsRef<str>) -> String {
    let c = content.as_ref();
    return c.replacen(
        &format!("{}. [ ]", item),
        &format!("{}. [x]", item),
        1
    );
}
fn uncheck_item(item: i32, content: impl AsRef<str>) -> String {
    let c = content.as_ref();
    return c.replacen(
        &format!("{}. [x]", item),
        &format!("{}. [ ]", item),
        1
    );
}
fn main() {
    let mut ctx = Args::parse();
    if ctx.default_list {
        ctx.check_env();
    }
    // print args if verbose
    ctx.v_print("==FLAGS==");
    ctx.v_print(format!("Add    : {}", ctx.add));
    ctx.v_print(format!("Check  : {}", ctx.check));
    ctx.v_print(format!("New    : {}", ctx.new));
    ctx.v_print(format!("Quiet  : {}", ctx.quiet));
    ctx.v_print(format!("Show   : {}", ctx.show));
    ctx.v_print(format!("Verbose: {}", ctx.verbose));
    ctx.v_print(format!("Uncheck: {}", ctx.uncheck));
    ctx.v_print("==/FLAGS==");
    ctx.v_print("==RUN==");
    // create new list
    if ctx.new && ctx.list_path.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        ctx.v_print(format!("Creating new list \"{}\"", &list));
        get_path(&mut ctx, &mut path, PathExitCondition::Exists);
        let mut content: String = NEW_CONTENT.clone().into();
        content = content.replace("[[C_DATE]]", &get_now());
        content = content.replace("[[E_DATE]]", &get_now());
        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(_) => ctx.c_exit(ExitCode::FileExists(&mut path)),
        };
        match file.write_all(content.as_bytes()) {
            Ok(_) => {},
            Err(_) => ctx.c_exit(ExitCode::FileExists(&mut path)),
        }
        ctx.v_print(format!("Created new list \"{}\"", &path.to_str().unwrap()));
    } else if ctx.new && ctx.list_path.is_none() {
        ctx.v_print("Missing name for list");
        ctx.c_exit(ExitCode::NoListName);
    }
    // add new list-item
    if ctx.add && ctx.list_path.is_some() && ctx.message.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        let msg = ctx.message.clone().unwrap();
        ctx.v_print(
            format!(
                "Creating new list-item in list \"{}\" with item-message \"{}\"",
                list,
                msg
            )
        );
        get_path(&mut ctx, &mut path, PathExitCondition::NotExists);
        let mut content = get_content_from_file(&mut ctx, &mut path);
        ctx.v_print("Discovering next item number");
        let num = get_next_item_number(&content);
        content.push_str(&format!("\n{}. [ ] {}", num, msg));
        content = update_last_edit(content, get_now());
        overwrite_content(&mut ctx, &mut path, content);
    } else if ctx.add && ctx.list_path.is_none() {
        ctx.c_exit(ExitCode::NoListName);
    } else if ctx.add && ctx.message.is_none() {
        let mut path = PathBuf::new();
        get_path(&mut ctx, &mut path, PathExitCondition::Ignore);
        ctx.c_exit(ExitCode::NoListItemMessage(&mut path));
    }
    // check list item
    if ctx.check && ctx.list_path.is_some() && ctx.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        get_path(&mut ctx, &mut path, PathExitCondition::NotExists);
        let item = ctx.item.clone().unwrap();
        ctx.v_print(format!("Checking item \"{}\" from list \"{}\"", item, list));
        let mut content = get_content_from_file(&mut ctx, &mut path);
        content = check_item(item, &content);
        content = update_last_edit(&content, Local::now().to_rfc3339());
        overwrite_content(&mut ctx, &mut path, content);
        ctx.v_print(format!("Checked item \"{}\" from list \"{}\"", item, list));
    } else if ctx.check && ctx.list_path.is_none() {
        ctx.c_exit(ExitCode::NoListName);
    } else if ctx.check && ctx.item.is_none() {
        let mut path = PathBuf::new();
        get_path(&mut ctx, &mut path, PathExitCondition::Ignore);
        ctx.c_exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.uncheck && ctx.list_path.is_some() && ctx.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        get_path(&mut ctx, &mut path, PathExitCondition::NotExists);
        let item = ctx.item.clone().unwrap();
        ctx.v_print(format!("Unchecking item \"{}\" from list \"{}\"", item, list));
        let mut content = get_content_from_file(&mut ctx, &mut path);
        content = uncheck_item(item, &content);
        content = update_last_edit(&content, Local::now().to_rfc3339());
        overwrite_content(&mut ctx, &mut path, content);
        ctx.v_print(format!("Unchecked item \"{}\" from list \"{}\"", item, list));
    } else if ctx.uncheck && ctx.list_path.is_none() {
        ctx.c_exit(ExitCode::NoListName);
    } else if ctx.uncheck && ctx.item.is_none() {
        let mut path = PathBuf::new();
        get_path(&mut ctx, &mut path, PathExitCondition::Ignore);
        ctx.c_exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.show && ctx.list_path.is_some() {
        let mut path = PathBuf::new();
        get_path(&mut ctx, &mut path, PathExitCondition::NotExists);
        let content = get_content_from_file(&mut ctx, &mut path);
        ctx.v_print("==FILE==");
        ctx.print(content);
        ctx.v_print("==/FILE==");
    }
    ctx.c_exit(ExitCode::Success);
}
