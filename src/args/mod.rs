use clap::Parser;
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
    // Flags
    /// Creates a new list-item from item-text
    #[clap(short, long)]
    pub add: bool,
    /// Checks off a list-item by item-number
    #[clap(short, long)]
    pub check: bool,
    /// Uses the list-name defined in env-var TODO_LIST when a name is not passed
    #[clap(short, long)]
    pub default_list: bool,
    /// Creates a new list
    #[clap(short, long)]
    pub new: bool,
    /// Displays list by name
    #[clap(short, long)]
    pub show: bool,
    /// Silences all messages (overrides verbose flag)
    #[clap(short, long)]
    pub quiet: bool,
    /// Unchecks a list-item by item-number
    #[clap(short, long)]
    pub uncheck: bool,
    /// Prints verbose messages during output
    #[clap(short, long)]
    pub verbose: bool,
    // Options
    /// Selects an item within a list or nested list by number
    #[clap(short, long)]
    pub item: Option<Vec<i32>>,
    /// Selects a list by path (w/o file extension)
    #[clap(short, long)]
    pub list_path: Option<String>,
    /// Adds an item to a list by message text
    #[clap(short, long)]
    pub message: Option<String>,
}
