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
    pub fn create(ctx: &mut Ctx, path: &mut PathBuf) -> Self {
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
    pub fn load(ctx: &mut Ctx, path: &mut PathBuf) -> Self {
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
    pub fn check_at(&mut self, indices: &mut Vec<i32>) {
        self.list.alter_check_at(true, indices);
    }
    pub fn uncheck_at(&mut self, indices: &mut Vec<i32>) {
        self.list.alter_check_at(false, indices);
    }
    pub fn save(&mut self, ctx: &mut Ctx) {
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
    pub fn printable(&mut self, content: &mut String) {
        self.list.printable(content);
    }
    pub fn add_item(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        self.list.add_item(indices, message);
    }
}
