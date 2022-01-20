use clap::Parser;
use clap::Subcommand;
use crate::enums::PrintWhich;
use crate::enums::ItemType;
#[derive(Parser, Clone)]
pub struct AddArgs {
    #[clap(short='i', long)]
    pub item_nest_location: Option<Vec<i32>>,
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
    Add(AddArgs),
    Check(CheckArgs),
    Edit(EditArgs),
    Move(MoveArgs),
    New,
    Show(ShowArgs),
    Remove(RemoveArgs),
    Uncheck(UncheckArgs),
}
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    // Options
    /// The relative or absolute path to the list (w/o file extension)
    #[clap(short='l', long)]
    pub list_path: String,
    // Flags
    /// Silences all messages (overrides verbose flag)
    #[clap(short='q', long)]
    pub quiet: bool,
    /// Prints verbose messages during output
    #[clap(short='v', long)]
    pub verbose: bool,
    // Modes
    #[clap(subcommand)]
    pub mode: Mode,
}
