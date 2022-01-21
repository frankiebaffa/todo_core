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
    fn construct_path(&mut self) {
        self.path.push(format!("{}.{}", &self.args.list_path, "json"));
    }
    pub fn init() -> Result<Self, ExitCode> {
        let mut args = Args::parse();
        args.reverse_coordinates();
        let buffer = String::new();
        let path = PathBuf::new();
        let mut ctx = Self { args, buffer, path, };
        ctx.construct_path();
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
}
