use clap::Parser;
use clap::Subcommand;
use crate::enums::ExitCode;
use crate::enums::ItemType;
use crate::enums::PrintWhich;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FormatError;
#[derive(Parser, Clone)]
pub struct AddArgs {
    #[clap(short='i', long)]
    pub item_nest_location: Vec<i32>,
    #[clap(short='m', long)]
    pub item_message: String,
    #[clap(short='t', long, default_value_t = ItemType::Todo)]
    pub item_type: ItemType,
}
#[derive(Parser, Clone)]
pub struct CheckArgs {
    #[clap(short='i', long)]
    pub item_location: Vec<i32>,
}
#[derive(Parser, Clone)]
pub struct EditArgs {
    #[clap(short='i', long)]
    pub item_location: Vec<i32>,
    #[clap(short='m', long)]
    pub item_message: String,
}
#[derive(Parser, Clone)]
pub struct MoveArgs {
    #[clap(short='i', long)]
    pub item_location: Vec<i32>,
    #[clap(short='o', long)]
    pub output_location: Vec<i32>,
}
#[derive(Parser, Clone)]
pub struct ShowArgs {
    #[clap(short='p', long, default_value_t = PrintWhich::All)]
    pub print_which: PrintWhich,
    #[clap(short='s', long)]
    pub status: bool,
    #[clap(long)]
    pub plain: bool,
}
#[derive(Parser, Clone)]
pub struct RemoveArgs {
    #[clap(short='i', long)]
    pub item_location: Vec<i32>,
}
#[derive(Parser, Clone)]
pub struct UncheckArgs {
    #[clap(short='i', long)]
    pub item_location: Vec<i32>,
}
#[derive(Subcommand, Clone)]
#[clap(about, version, author)]
pub enum Mode {
    /// Add a new list-item
    Add(AddArgs),
    /// Check-off an existing list-item
    Check(CheckArgs),
    /// Edit the item-text of an existing list-item
    Edit(EditArgs),
    /// Move an existing list-item to a new location
    Move(MoveArgs),
    /// Create a new list
    New,
    /// Show an existing list
    Show(ShowArgs),
    /// Remove an existing list-item
    Remove(RemoveArgs),
    /// Uncheck an existing list-item
    Uncheck(UncheckArgs),
}
impl Display for Mode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            Mode::Add(_) => fmt.write_str("Add"),
            Mode::Check(_) => fmt.write_str("Check"),
            Mode::Edit(_) => fmt.write_str("Edit"),
            Mode::Move(_) => fmt.write_str("Move"),
            Mode::New => fmt.write_str("New"),
            Mode::Show(_) => fmt.write_str("Show"),
            Mode::Remove(_) => fmt.write_str("Remove"),
            Mode::Uncheck(_) => fmt.write_str("Uncheck"),
        }
    }
}
impl Mode {
    fn reverse_coordinates(mut self) -> Self {
        match self {
            Mode::Add(mut mode_args) => {
                mode_args.item_nest_location.reverse();
                self = Mode::Add(AddArgs::from(mode_args.clone()));
            },
            Mode::Check(mut mode_args) => {
                mode_args.item_location.reverse();
                self = Mode::Check(CheckArgs::from(mode_args.clone()));
            },
            Mode::Edit(mut mode_args) => {
                mode_args.item_location.reverse();
                self = Mode::Edit(EditArgs::from(mode_args.clone()));
            },
            Mode::Move(mut mode_args) => {
                mode_args.item_location.reverse();
                self = Mode::Move(MoveArgs::from(mode_args.clone()));
            },
            Mode::Remove(mut mode_args) => {
                mode_args.item_location.reverse();
                self = Mode::Remove(RemoveArgs::from(mode_args.clone()));
            },
            Mode::Uncheck(mut mode_args) => {
                mode_args.item_location.reverse();
                self  = Mode::Uncheck(UncheckArgs::from(mode_args.clone()));
            },
            _ => {},
        }
        self
    }
}
fn safe_get_list(arg: &str) -> Result<String, String> {
    if arg.is_empty() {
        match std::env::var("TODO_LIST") {
            Ok(s) => Ok(s),
            Err(_) => return Err(ExitCode::NoEnvVar.to_string()),
        }
    } else {
        Ok(arg.to_string())
    }
}
/// A todo list manager
#[derive(Parser, Clone)]
#[clap(about, version, author)]
pub struct Args {
    // Options
    // Make the list_path arg require either a string passed or the TODO_LIST env var
    /// The relative or absolute path to the list (w/o file extension)
    #[clap(short='l', long="list-path", default_value_t = String::new(), parse(try_from_str = safe_get_list))]
    pub list_path: String,
    // Flags
    /// Silences all messages (overrides verbose flag)
    #[clap(short='q', long)]
    pub quiet: bool,
    /// Prints verbose messages during output
    #[clap(short='v', long)]
    pub verbose: bool,
    // Modes
    /// The program action to take
    #[clap(subcommand)]
    pub mode: Mode,
}
impl Args {
    pub fn reverse_coordinates(mut self) -> Self {
        self.mode = self.mode.reverse_coordinates();
        return self.clone();
    }
}
