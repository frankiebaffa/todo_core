use chrono::DateTime;
use chrono::Local;
use clap::Parser;
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
    /// Selects an item within a list or nested list by number
    #[clap(short, long)]
    item: Option<Vec<i32>>,
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
    fn exit(&mut self, code: ExitCode) -> ! {
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
                        self.exit(ExitCode::FileExists(path));
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
                        self.exit(ExitCode::FileDoesNotExist(path));
                    }
                },
                PathExitCondition::Ignore => {},
            }
        } else {
            self.exit(ExitCode::NoListName);
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
    fn new(text: impl AsRef<str>) -> Self {
        let txt = text.as_ref().to_string();
        Self {
            checked: false,
            text: txt,
            sub_items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
        }
    }
    fn alter_check(&mut self, checked: bool, indices: &mut Vec<i32>) {
        if indices.len().eq(&0) {
            self.checked = checked;
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.alter_check(checked, indices);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    fn add_item(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        if indices.len().eq(&0) {
            self.sub_items.push(Self::new(message));
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.add_item(indices, message);
                    break;
                }
                iter_c = iter_c + 1;
            }
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
        let mut sub_index = 1;
        for sub in self.sub_items.iter() {
            sub.printable(content, &mut sub_index, &mut (level.add(1)));
            sub_index = sub_index + 1;
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
                ctx.exit(ExitCode::FailedToDeserialize(e));
            },
        };
        list
    }
    fn to_json(&self, ctx: &mut Args) -> String {
        let json = match to_json_string(self) {
            Ok(json) => json,
            Err(e) => {
                ctx.exit(ExitCode::FailedToSerialize(e));
            },
        };
        json
    }
    fn printable(&mut self, content: &mut String) {
        let created = self.created.format("%m/%d/%Y %H:%M:%S");
        let updated = self.last_updated.format("%m/%d/%Y %H:%M:%S");
        content.push_str(
            &format!(concat!(
                "Created On: {}\n",
                "Last Edit : {}\n",
                "\n",
            ), created, updated)
        );
        let mut level = 0;
        let mut index = 1;
        for item in self.items.iter() {
            item.printable(content, &mut index, &mut level);
            index = index.add(1);
        }
    }
    fn alter_check_at(&mut self, checked: bool, indices: &mut Vec<i32>) {
        let list_item_index = indices.pop().unwrap();
        let mut iter_c = 1;
        for act_item in self.items.iter_mut() {
            if iter_c.eq(&list_item_index) {
                act_item.alter_check(checked, indices);
                self.last_updated = Local::now();
                break;
            }
            iter_c = iter_c + 1;
        }
    }
    fn add_item(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        if indices.len().eq(&0) {
            self.items.push(Item::new(message));
            self.last_updated = Local::now();
        } else {
            let list_item_index = indices.pop().unwrap();
            let mut iter_c = 1;
            for act_item in self.items.iter_mut() {
                if iter_c.eq(&list_item_index) {
                    act_item.add_item(indices, message);
                    self.last_updated = Local::now();
                    break;
                }
                iter_c = iter_c + 1;
            }
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
            ctx.exit(ExitCode::FileExists(path));
        }
        { // file creation
            match File::create(&path) {
                Ok(_) => {},
                Err(_) => {
                    ctx.exit(ExitCode::FailedToOpen(path));
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
            ctx.exit(ExitCode::FileExists(path));
        }
        let mut json = String::new();
        { // file read
            let mut file = match OpenOptions::new()
                .read(true)
                .open(&path)
            {
                Ok(f) => f,
                Err(_) => ctx.exit(ExitCode::FailedToOpen(path)),
            };
            match file.read_to_string(&mut json) {
                Ok(_) => {},
                Err(_) => {
                    ctx.exit(ExitCode::FailedToRead(path));
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
    fn check_at(&mut self, indices: &mut Vec<i32>) {
        self.list.alter_check_at(true, indices);
    }
    fn uncheck_at(&mut self, indices: &mut Vec<i32>) {
        self.list.alter_check_at(false, indices);
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
                Err(_) => ctx.exit(ExitCode::FailedToOpen(&mut self.path)),
            };
            match file.write_all(bytes) {
                Ok(_) => {},
                Err(_) => {
                    ctx.exit(ExitCode::FailedToWrite(&mut self.path));
                },
            }
        } // file locked
    }
    fn printable(&mut self, content: &mut String) {
        self.list.printable(content);
    }
    fn add_item(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        self.list.add_item(indices, message);
    }
}
fn get_printable_coords(nums: &Vec<i32>) -> String {
    nums.into_iter().map(|num| {
        num.to_string()
    }).collect::<Vec<String>>().join(", ").to_string()
}
fn main() {
    let mut ctx = Args::parse();
    if ctx.default_list {
        ctx.check_env();
    }
    // reverse vector so that pop works
    if ctx.item.is_some() {
        let mut item = ctx.item.unwrap();
        item.reverse();
        ctx.item = Some(item);
    }
    // print args if verbose
    ctx.v_print("==FLAGS==");
    ctx.v_print(format!("Add      : {}", ctx.add));
    ctx.v_print(format!("Check    : {}", ctx.check));
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
        ctx.get_path(&mut path, "json", PathExitCondition::Exists);
        let mut container = Container::create(&mut ctx, &mut path);
        container.save(&mut ctx);
        ctx.v_print(format!("Created new list \"{}\"", &path.to_str().unwrap()));
    } else if ctx.new && ctx.list_path.is_none() {
        ctx.v_print("Missing name for list");
        ctx.exit(ExitCode::NoListName);
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
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        let mut container = Container::load(&mut ctx, &mut path);
        let item = if ctx.item.is_some() {
            ctx.item.clone().unwrap()
        } else {
            Vec::new()
        };
        container.add_item(&mut item.clone(), msg);
        container.save(&mut ctx);
    } else if ctx.add && ctx.list_path.is_none() {
        ctx.exit(ExitCode::NoListName);
    } else if ctx.add && ctx.message.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.exit(ExitCode::NoListItemMessage(&mut path));
    }
    // check list item
    if ctx.check && ctx.list_path.is_some() && ctx.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        let item = ctx.item.clone().unwrap();
        ctx.v_print(format!("Checking item \"{}\" from list \"{}\"", get_printable_coords(&item), list));
        let mut container = Container::load(&mut ctx, &mut path);
        container.check_at(&mut item.clone());
        container.save(&mut ctx);
        ctx.v_print(format!("Checked from list \"{}\"", list));
    } else if ctx.check && ctx.list_path.is_none() {
        ctx.exit(ExitCode::NoListName);
    } else if ctx.check && ctx.item.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.uncheck && ctx.list_path.is_some() && ctx.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.list_path.clone().unwrap();
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        let item = ctx.item.clone().unwrap();
        ctx.v_print(format!("Unchecking item \"{}\" from list \"{}\"", get_printable_coords(&item), list));
        let mut container = Container::load(&mut ctx, &mut path);
        container.uncheck_at(&mut item.clone());
        container.save(&mut ctx);
        ctx.v_print(format!("Unchecked from list \"{}\"", list));
    } else if ctx.uncheck && ctx.list_path.is_none() {
        ctx.exit(ExitCode::NoListName);
    } else if ctx.uncheck && ctx.item.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.show && ctx.list_path.is_some() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        ctx.v_print("==FILE==");
        let mut content = String::new();
        let mut container = Container::load(&mut ctx, &mut path);
        container.printable(&mut content);
        ctx.print(content);
        ctx.v_print("==/FILE==");
    }
    ctx.exit(ExitCode::Success);
}
