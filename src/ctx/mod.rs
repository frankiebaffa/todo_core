use clap::Parser;
use crate::args::Args;
use crate::enums::ExitCode;
use crate::enums::PathExitCondition;
use std::path::PathBuf;
pub struct Ctx {
    pub args: Args,
    pub buffer: String,
    pub path: PathBuf,
}
impl<'ctx> Ctx {
    fn check_env(args: &mut Args) {
        match args.list_path {
            Some(_) => {},
            None => {
                match std::env::var("TODO_LIST") {
                    Ok(v) => args.list_path = Some(v),
                    Err(_) => {},
                }
            },
        }
    }
    fn construct_path(&mut self) -> Result<(), ExitCode> {
        if self.args.list_path.is_some() {
            let list = self.args.list_path.clone().unwrap();
            self.path.push(format!("{}.{}", &list, "json"));
            return Ok(());
        } else {
            return Err(ExitCode::NoListName);
        }
    }
    pub fn init() -> Result<Self, ExitCode> {
        let mut args = Args::parse();
        if args.default_list {
            Self::check_env(&mut args);
        }
        let buffer = String::new();
        let path = PathBuf::new();
        let mut ctx = Self { args, buffer, path, };
        ctx.construct_path()?;
        // reverse vector so that pop works
        if ctx.args.item.is_some() {
            let mut item = ctx.args.item.unwrap();
            item.reverse();
            ctx.args.item = Some(item);
        }
        Ok(ctx)
    }
    pub fn check_path(&mut self, condition: PathExitCondition) -> Result<(), ExitCode> {
        match condition {
            PathExitCondition::Exists => {
                if self.path.exists() {
                    return Err(ExitCode::FileExists(self.path.clone()));
                } else {
                    return Ok(());
                }
            },
            PathExitCondition::NotExists => {
                if !self.path.exists() {
                    self.v_print(
                        format!(
                            "File at \"{}\" does not exist",
                            &self.path.to_str().unwrap()
                        )
                    );
                    return Err(ExitCode::FileDoesNotExist(self.path.clone()));
                } else {
                    return Ok(());
                }
            },
            PathExitCondition::Ignore => return Ok(()),
        };
    }
    pub fn print(&mut self, msg: impl AsRef<str>) {
        if !self.buffer.is_empty() {
            // TODO: Fix buffer line breaking
            self.buffer.push_str("\n");
        }
        self.buffer.push_str(&format!("{}", msg.as_ref()));
    }
    pub fn q_print(&mut self, msg: impl AsRef<str>) {
        if !self.args.quiet {
            self.print(msg);
        }
    }
    pub fn v_print(&mut self, msg: impl AsRef<str>) {
        if self.args.verbose {
            self.q_print(msg);
        }
    }
    pub fn flush(&mut self, code: &ExitCode) {
        self.v_print(format!("{}", code));
        if !self.buffer.is_empty() {
            println!("{}", self.buffer);
        }
    }
    /// Checks if new list should be created
    pub fn new_list_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.new && self.args.list_path.is_some() {
            Ok(true)
        } else if self.args.new && self.args.list_path.is_none() {
            self.v_print("Missing name for list");
            Err(ExitCode::NoListName)
        } else {
            Ok(false)
        }
    }
    /// Checks if new list item should be created
    pub fn new_item_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.add && self.args.list_path.is_some() && self.args.message.is_some() {
            Ok(true)
        } else if self.args.add && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else if self.args.add && self.args.message.is_none() {
            Err(ExitCode::NoListItemMessage(self.path.clone()))
        } else {
            Ok(false)
        }
    }
    /// Checks if existing list item should be checked
    pub fn check_item_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.check && self.args.list_path.is_some() && self.args.item.is_some() {
            Ok(true)
        } else if self.args.check && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else if self.args.check && self.args.item.is_none() {
            Err(ExitCode::NoListItemNumber(self.path.clone()))
        } else {
            Ok(false)
        }
    }
    /// Checks if existing list item should be checked
    pub fn uncheck_item_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.uncheck && self.args.list_path.is_some() && self.args.item.is_some() {
            Ok(true)
        } else if self.args.uncheck && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else if self.args.uncheck && self.args.item.is_none() {
            Err(ExitCode::NoListItemNumber(self.path.clone()))
        } else {
            Ok(false)
        }
    }
    /// Checks if list should be displayed
    pub fn show_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.show && self.args.list_path.is_some() {
            Ok(true)
        } else if self.args.show && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else {
            Ok(false)
        }
    }
    /// Checks if list item should be removed
    pub fn remove_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.remove && self.args.list_path.is_some() && self.args.item.is_some() {
            Ok(true)
        } else if self.args.remove && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else if self.args.remove && self.args.item.is_none() {
            Err(ExitCode::NoListItemNumber(self.path.clone()))
        } else {
            Ok(false)
        }
    }
    /// Checks if list item should be edited
    pub fn edit_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.edit && self.args.list_path.is_some() &&
            self.args.item.is_some() && self.args.message.is_some()
        {
            Ok(true)
        } else if self.args.edit && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else if self.args.edit && self.args.item.is_none() {
            Err(ExitCode::NoListItemNumber(self.path.clone()))
        } else if self.args.edit && self.args.message.is_none() {
            Err(ExitCode::NoListItemMessage(self.path.clone()))
        } else {
            Ok(false)
        }
    }
    /// Checks if the list status should be displayed
    pub fn status_mode(&mut self) -> Result<bool, ExitCode> {
        if self.args.status && self.args.list_path.is_some() {
            Ok(true)
        } else if self.args.status && self.args.list_path.is_none() {
            Err(ExitCode::NoListName)
        } else {
            Ok(false)
        }
    }
}
