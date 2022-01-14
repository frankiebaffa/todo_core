use crate::ctx::Ctx;
use crate::enums::ExitCode;
use crate::list::List;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
pub struct Container {
    pub path: PathBuf,
    pub list: List,
}
impl Container {
    pub fn create(ctx: &mut Ctx) -> Result<Self, ExitCode> {
        if ctx.path.exists() {
            return Err(ExitCode::FileExists(ctx.path.clone()));
        }
        { // file creation
            match File::create(&ctx.path) {
                Ok(_) => {},
                Err(_) => return Err(ExitCode::FailedToOpen(ctx.path.clone())),
            }
        } // file unlocked
        let name = ctx.path.file_name().unwrap().to_str().unwrap().to_string();
        let list = List::new(name);
        Ok(Self {
            path: ctx.path.clone(),
            list,
        })
    }
    pub fn load(ctx: &mut Ctx) -> Result<Self, ExitCode> {
        let path = ctx.path.clone();
        let mut json = String::new();
        { // file read
            let mut file = match OpenOptions::new()
                .read(true)
                .open(&mut ctx.path)
            {
                Ok(f) => f,
                Err(_) => return Err(ExitCode::FailedToOpen(ctx.path.clone())),
            };
            match file.read_to_string(&mut json) {
                Ok(_) => {},
                Err(_) => {
                    return Err(ExitCode::FailedToRead(ctx.path.clone()));
                },
            }
        } // file locked
        let mut list = List::from_json(json)?;
        list.items.sort_by(|a, b| {
            a.created.cmp(&b.created)
        });
        Ok(Self {
            path,
            list,
        })
    }
    pub fn check_at(&mut self, indices: &mut Vec<i32>) {
        self.list.alter_check_at(true, indices);
    }
    pub fn uncheck_at(&mut self, indices: &mut Vec<i32>) {
        self.list.alter_check_at(false, indices);
    }
    pub fn edit_at(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        self.list.edit_at(indices, message);
    }
    pub fn remove_at(&mut self, indices: &mut Vec<i32>) {
        self.list.remove_at(indices);
    }
    pub fn save(&mut self) -> Result<(), ExitCode> {
        let json = self.list.to_json()?;
        { // file open:write
            let bytes = json.as_bytes();
            let mut file = match OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&self.path)
            {
                Ok(f) => f,
                Err(_) => return Err(ExitCode::FailedToOpen(self.path.clone())),
            };
            match file.write_all(bytes) {
                Ok(_) => {},
                Err(_) => return Err(ExitCode::FailedToWrite(self.path.clone())),
            }
        } // file locked
        Ok(())
    }
    pub fn printable(&mut self, content: &mut String) {
        self.list.printable(content);
    }
    pub fn add_item(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        self.list.add_item(indices, message);
    }
}