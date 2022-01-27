use std::path::PathBuf;
pub trait GetPath {
    fn get_path(&self) -> &PathBuf;
    fn get_path_mut(&mut self) -> &mut PathBuf;
}
