use {
    crossterm::QueueableCommand,
    std::{
        io::Error as IOError,
        path::PathBuf,
    },
};
pub trait GetPath {
    fn get_path(&self) -> &PathBuf;
    fn get_path_mut(&mut self) -> &mut PathBuf;
}
pub trait Terminal
where
    Self: std::io::Write
{
    fn queue_cmd(&self, cmd: impl QueueableCommand) -> Result<(), IOError>;
    fn write_str(&self, msg: impl AsRef<str>) -> Result<(), IOError>;
}
