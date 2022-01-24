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
    #[clap()]
    pub item_nest_location: Vec<usize>,
    #[clap(short='m', long)]
    pub item_message: String,
    #[clap(short='t', long, default_value_t = ItemType::Todo)]
    pub item_type: ItemType,
}
#[derive(Parser, Clone)]
pub struct CheckArgs {
    #[clap()]
    pub item_location: Vec<usize>,
}
#[derive(Parser, Clone)]
pub struct DisableArgs {
    #[clap()]
    pub item_location: Vec<usize>,
}
#[derive(Parser, Clone)]
pub struct EditArgs {
    #[clap()]
    pub item_location: Vec<usize>,
    #[clap(short='m', long)]
    pub item_message: String,
}
#[derive(Parser, Clone)]
pub struct MoveArgs {
    #[clap()]
    pub item_location: Vec<usize>,
    #[clap(short='o', long)]
    pub output_location: Vec<usize>,
}
#[derive(Parser, Clone)]
pub struct ShowArgs {
    #[clap(short='p', long, default_value_t = PrintWhich::All)]
    pub print_which: PrintWhich,
    #[clap(short='s', long)]
    pub status: bool,
    #[clap(long)]
    pub plain: bool,
    #[clap(short, long)]
    pub level: Option<usize>,
}
#[derive(Parser, Clone)]
pub struct RemoveArgs {
    #[clap()]
    pub item_location: Vec<usize>,
}
#[derive(Parser, Clone)]
pub struct UncheckArgs {
    #[clap()]
    pub item_location: Vec<usize>,
}
#[derive(Subcommand, Clone)]
#[clap(about, version, author)]
pub enum Mode {
    /// Add a new list-item
    Add(AddArgs),
    /// Check-off an existing list-item
    Check(CheckArgs),
    /// Disable an existing list-item
    Disable(DisableArgs),
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
            Mode::Disable(_) => fmt.write_str("Disable"),
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
    fn reverse_coordinates(&mut self) {
        match self {
            &mut Mode::Add(ref mut mode_args) => {
                mode_args.item_nest_location.reverse();
            },
            &mut Mode::Check(ref mut mode_args) => {
                mode_args.item_location.reverse();
            },
            &mut Mode::Disable(ref mut mode_args) => {
                mode_args.item_location.reverse();
            },
            &mut Mode::Edit(ref mut mode_args) => {
                mode_args.item_location.reverse();
            },
            &mut Mode::Move(ref mut mode_args) => {
                mode_args.item_location.reverse();
            },
            &mut Mode::Remove(ref mut mode_args) => {
                mode_args.item_location.reverse();
            },
            &mut Mode::Uncheck(ref mut mode_args) => {
                mode_args.item_location.reverse();
            },
            &mut Mode::New | &mut Mode::Show(_) => {},
        }
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
    pub fn reverse_coordinates(&mut self) {
        self.mode.reverse_coordinates();
    }
}
