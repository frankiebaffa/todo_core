use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string as to_json_string;
use serde_json::from_str as from_json_string;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FormatError;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;
use std::process::exit;
const NEW_CONTENT: &'static str = include_str!("../docs/new.md");
#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    // Flags
    /// Changes file-type mode to markdown
    #[clap(long)]
    markdown_mode: bool,
    /// Changes file-type mode to json
    #[clap(long)]
    json_mode: bool,
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
    fn get_path(&mut self, path: &mut PathBuf, ext: impl AsRef<str>, chk: PathExitCondition) {
        if self.list_path.is_some() {
            let list = self.list_path.clone().unwrap();
            path.push(format!("{}.{}", &list, ext.as_ref()));
            match chk {
                PathExitCondition::Exists => {
                    if path.exists() {
                        self.c_exit(ExitCode::FileExists(path));
                    }
                },
                PathExitCondition::NotExists => {
                    if !path.exists() {
                        self.v_print(
                            format!(
                                "File at \"{}\" does not exist",
                                &path.to_str().unwrap()
                            )
                        );
                        self.c_exit(ExitCode::FileDoesNotExist(path));
                    }
                },
                PathExitCondition::Ignore => {},
            }
        } else {
            self.c_exit(ExitCode::NoListName);
        }
    }
    fn is_markdown_mode(&mut self) -> bool {
        if self.markdown_mode && self.json_mode {
            self.c_exit(ExitCode::TooManyModes);
        } else if (!self.markdown_mode && !self.json_mode) || self.markdown_mode {
            true
        } else {
            false
        }
    }
    fn get_ext(&mut self) -> String {
        if self.is_markdown_mode() {
            "md".to_owned()
        } else {
            "json".to_owned()
        }
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
    FailedToDeserialize(serde_json::Error),
    FailedToSerialize(serde_json::Error),
    TooManyModes,
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
            Self::FailedToDeserialize(_) => 13,
            Self::FailedToSerialize(_) => 14,
            Self::TooManyModes => 15,
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
            Self::FailedToDeserialize(e) => {
                return f.write_str(&format!("Failed to deserialize json: {}", e));
            },
            Self::FailedToSerialize(e) => {
                return f.write_str(&format!("Failed to serialize to json: {}", e));
            },
            Self::TooManyModes => {
                return f.write_str("Only one mode flag can be set");
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
    checked: bool,
    text: String,
    sub_items: Vec<Item>,
    created: DateTime<Local>,
    last_updated: DateTime<Local>,
}
impl Item {
    fn new(text: String) -> Self {
        Self {
            checked: false,
            text,
            sub_items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
        }
    }
    fn printable(&self, content: &mut String, index: &mut usize, level: &mut usize) {
        let mut indent = String::new();
        for _ in 0..level.clone() {
            indent.push_str("    ");
        }
        let chked = if self.checked {
            "[x]"
        } else {
            "[ ]"
        };
        content.push_str(&format!(concat!(
            "{}{}. {} {}\n"
        ), indent, index, chked, self.text));
        for sub in self.sub_items.iter() {
            sub.printable(content, &mut 0, &mut (level.add(1)));
        }
    }
}
#[derive(Serialize, Deserialize)]
struct List {
    name: String,
    items: Vec<Item>,
    created: DateTime<Local>,
    last_updated: DateTime<Local>,
}
impl List {
    fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
        }
    }
    fn from_json(ctx: &mut Args, json: String) -> Self {
        let list = match from_json_string(&json) {
            Ok(list) => list,
            Err(e) => {
                ctx.c_exit(ExitCode::FailedToDeserialize(e));
            },
        };
        list
    }
    fn to_json(&self, ctx: &mut Args) -> String {
        let json = match to_json_string(self) {
            Ok(json) => json,
            Err(e) => {
                ctx.c_exit(ExitCode::FailedToSerialize(e));
            },
        };
        json
    }
    fn printable(&mut self, content: &mut String) {
        content.push_str(
            &format!(concat!(
                "Created On: {}\n",
                "Last Edit : {}\n",
                "\n",
            ), self.created.to_rfc3339(), self.last_updated.to_rfc3339())
        );
        let mut level = 0;
        let mut index = 1;
        for item in self.items.iter() {
            item.printable(content, &mut index, &mut level);
            index = index.add(1);
        }
    }
}
struct Container {
    path: PathBuf,
    list: List,
}
impl Container {
    fn create(ctx: &mut Args, path: &mut PathBuf) -> Self {
        if path.exists() {
            ctx.c_exit(ExitCode::FileExists(path));
        }
        { // file creation
            match File::create(&path) {
                Ok(_) => {},
                Err(_) => {
                    ctx.c_exit(ExitCode::FailedToOpen(path));
                },
            };
        } // file unlocked
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let list = List::new(name);
        Self {
            path: path.clone(),
            list,
        }
    }
    fn load(ctx: &mut Args, path: &mut PathBuf) -> Self {
        if !path.exists() {
            ctx.c_exit(ExitCode::FileExists(path));
        }
        let mut json = String::new();
        { // file read
            let mut file = match OpenOptions::new()
                .read(true)
                .open(&path)
            {
                Ok(f) => f,
                Err(_) => ctx.c_exit(ExitCode::FailedToOpen(path)),
            };
            match file.read_to_string(&mut json) {
                Ok(_) => {},
                Err(_) => {
                    ctx.c_exit(ExitCode::FailedToRead(path));
                },
            }
        } // file locked
        let mut list = List::from_json(ctx, json);
        list.items.sort_by(|a, b| {
            a.created.cmp(&b.created)
        });
        Self {
            path: path.clone(),
            list,
        }
    }
    fn save(&mut self, ctx: &mut Args) {
        let json = self.list.to_json(ctx);
        { // file open:write
            let bytes = json.as_bytes();
            let mut file = match OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&self.path)
            {
                Ok(f) => f,
                Err(_) => ctx.c_exit(ExitCode::FailedToOpen(&mut self.path)),
            };
            match file.write_all(bytes) {
                Ok(_) => {},
                Err(_) => {
                    ctx.c_exit(ExitCode::FailedToWrite(&mut self.path));
                },
            }
        } // file locked
    }
    fn printable(&mut self, content: &mut String) {
        self.list.printable(content);
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
    ctx.v_print(format!("Add      : {}", ctx.add));
    ctx.v_print(format!("Check    : {}", ctx.check));
    ctx.v_print(format!("JSON Mode: {}", ctx.json_mode));
    ctx.v_print(format!("MD Mode  : {}", ctx.markdown_mode));
    ctx.v_print(format!("New      : {}", ctx.new));
    ctx.v_print(format!("Quiet    : {}", ctx.quiet));
    ctx.v_print(format!("Show     : {}", ctx.show));
    ctx.v_print(format!("Verbose  : {}", ctx.verbose));
    ctx.v_print(format!("Uncheck  : {}", ctx.uncheck));
    ctx.v_print("==/FLAGS==");
    ctx.v_print("==RUN==");
    // create new list
    if ctx.new && ctx.list_path.is_some() {
        let list = ctx.list_path.clone().unwrap();
        ctx.v_print(format!("Creating new list \"{}\"", &list));
        let mut path = PathBuf::new();
        let ext = ctx.get_ext();
        ctx.get_path(&mut path, &ext, PathExitCondition::Exists);
        if ctx.is_markdown_mode() {
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
        } else {
            let mut container = Container::create(&mut ctx, &mut path);
            container.save(&mut ctx);
        }
        ctx.v_print(format!("Created new list \"{}\"", &path.to_str().unwrap()));
    } else if ctx.new && ctx.list_path.is_none() {
        ctx.v_print("Missing name for list");
        ctx.c_exit(ExitCode::NoListName);
    }
    // add new list-item
    if ctx.add && ctx.list_path.is_some() && ctx.message.is_some() {
        let mut path = PathBuf::new();
        let ext = ctx.get_ext();
        let list = ctx.list_path.clone().unwrap();
        let msg = ctx.message.clone().unwrap();
        ctx.v_print(
            format!(
                "Creating new list-item in list \"{}\" with item-message \"{}\"",
                list,
                msg
            )
        );
        ctx.get_path(&mut path, &ext, PathExitCondition::NotExists);
        if ctx.is_markdown_mode() {
            let mut content = get_content_from_file(&mut ctx, &mut path);
            let num = get_next_item_number(&content);
            content.push_str(&format!("\n{}. [ ] {}", num, msg));
            content = update_last_edit(content, get_now());
            overwrite_content(&mut ctx, &mut path, content);
        } else {
            let mut container = Container::load(&mut ctx, &mut path);
            container.list.items.push(Item::new(msg));
            container.save(&mut ctx);
        }
    } else if ctx.add && ctx.list_path.is_none() {
        ctx.c_exit(ExitCode::NoListName);
    } else if ctx.add && ctx.message.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.c_exit(ExitCode::NoListItemMessage(&mut path));
    }
    // check list item
    if ctx.check && ctx.list_path.is_some() && ctx.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        let ext = ctx.get_ext();
        ctx.get_path(&mut path, ext, PathExitCondition::NotExists);
        let item = ctx.item.clone().unwrap();
        ctx.v_print(format!("Checking item \"{}\" from list \"{}\"", item, list));
        if ctx.is_markdown_mode() {
            let mut content = get_content_from_file(&mut ctx, &mut path);
            content = check_item(item, &content);
            content = update_last_edit(&content, Local::now().to_rfc3339());
            overwrite_content(&mut ctx, &mut path, content);
        } else {
            let mut container = Container::load(&mut ctx, &mut path);
            let mut iter_c = 1;
            for act_item in container.list.items.iter_mut() {
                if iter_c.eq(&item) {
                    act_item.checked = true;
                    break;
                }
                iter_c = iter_c + 1;
            }
            container.save(&mut ctx);
        }
        ctx.v_print(format!("Checked item \"{}\" from list \"{}\"", item, list));
    } else if ctx.check && ctx.list_path.is_none() {
        ctx.c_exit(ExitCode::NoListName);
    } else if ctx.check && ctx.item.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.c_exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.uncheck && ctx.list_path.is_some() && ctx.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        let ext = ctx.get_ext();
        ctx.get_path(&mut path, ext, PathExitCondition::NotExists);
        let item = ctx.item.clone().unwrap();
        ctx.v_print(format!("Unchecking item \"{}\" from list \"{}\"", item, list));
        if ctx.is_markdown_mode() {
            let mut content = get_content_from_file(&mut ctx, &mut path);
            content = uncheck_item(item, &content);
            content = update_last_edit(&content, Local::now().to_rfc3339());
            overwrite_content(&mut ctx, &mut path, content);
        } else {
            let mut container = Container::load(&mut ctx, &mut path);
            let mut iter_c = 1;
            for act_item in container.list.items.iter_mut() {
                if iter_c.eq(&item) {
                    act_item.checked = false;
                    break;
                }
                iter_c = iter_c + 1;
            }
            container.save(&mut ctx);
        }
        ctx.v_print(format!("Unchecked item \"{}\" from list \"{}\"", item, list));
    } else if ctx.uncheck && ctx.list_path.is_none() {
        ctx.c_exit(ExitCode::NoListName);
    } else if ctx.uncheck && ctx.item.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.c_exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.show && ctx.list_path.is_some() {
        let mut path = PathBuf::new();
        let ext = ctx.get_ext();
        ctx.get_path(&mut path, ext, PathExitCondition::NotExists);
        ctx.v_print("==FILE==");
        let mut content = String::new();
        if ctx.is_markdown_mode() {
            content = get_content_from_file(&mut ctx, &mut path);
        } else {
            let mut container = Container::load(&mut ctx, &mut path);
            container.printable(&mut content);
        }
        ctx.print(content);
        ctx.v_print("==/FILE==");
    }
    ctx.c_exit(ExitCode::Success);
}
