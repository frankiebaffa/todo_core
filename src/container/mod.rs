use {
    crate::{
        traits::{
            GetPath,
            Terminal,
        },
        enums::{
            ExitCode,
            PrintWhich,
        },
        list::List,
    },
    std::{
        fs::{
            File,
            OpenOptions,
        },
        io::{
            Write,
            Read,
            Error as IOError,
        },
        path::PathBuf,
    },
};
pub struct Container {
    pub path: PathBuf,
    pub list: List,
}
impl Container {
    pub fn create(ctx: &mut impl GetPath) -> Result<Self, ExitCode> {
        let path = ctx.get_path_mut();
        if path.exists() {
            return Err(ExitCode::FileExists(path.clone()));
        }
        { // file creation
            match File::create(&path) {
                Ok(_) => {},
                Err(_) => return Err(ExitCode::FailedToOpen(path.clone())),
            }
        } // file unlocked
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let list = List::new(name);
        Ok(Self {
            path: path.clone(),
            list,
        })
    }
    pub fn load(ctx: &mut impl GetPath) -> Result<Self, ExitCode> {
        let mut path = ctx.get_path_mut().clone();
        let mut json = String::new();
        { // file read
            let mut file = match OpenOptions::new()
                .read(true)
                .open(&mut path)
            {
                Ok(f) => f,
                Err(_) => return Err(ExitCode::FailedToOpen(path.clone())),
            };
            match file.read_to_string(&mut json) {
                Ok(_) => {},
                Err(_) => {
                    return Err(ExitCode::FailedToRead(path.clone()));
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
    pub fn print(
        &mut self, ctx: &mut impl Terminal, print_which: &PrintWhich, plain: bool,
        max_level: Option<usize>, display_hidden: bool,
    ) -> Result<(), IOError> {
        self.list.print(ctx, print_which, plain, max_level, display_hidden)
    }
    pub fn status(&mut self, content: &mut String, print_which: &PrintWhich) {
        self.list.status(content, print_which);
    }
}
