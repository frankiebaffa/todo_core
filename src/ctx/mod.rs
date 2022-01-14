use clap::Parser;
use crate::args::Args;
use crate::enums::ExitCode;
use crate::enums::PathExitCondition;
use std::path::PathBuf;
use std::process::exit;
pub struct Ctx {
    pub args: Args,
    pub buffer: String,
}
impl Ctx {
    pub fn init() -> Self {
        let args = Args::parse();
        let buffer = String::new();
        Self { args, buffer, }
    }
    pub fn check_env(&mut self) {
        match self.args.list_path {
            Some(_) => {},
            None => {
                match std::env::var("TODO_LIST") {
                    Ok(v) => self.args.list_path = Some(v),
                    Err(_) => {},
                }
            },
        }
    }
    pub fn print(&mut self, msg: impl AsRef<str>) {
        self.buffer.push_str(&format!("\n{}", msg.as_ref()));
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
    pub fn exit(&mut self, code: ExitCode) -> ! {
        self.v_print(format!("{}", code));
        if !self.buffer.is_empty() {
            println!("{}", self.buffer);
        }
        exit(code.into());
    }
    pub fn get_path(&mut self, path: &mut PathBuf, ext: impl AsRef<str>, chk: PathExitCondition) {
        if self.args.list_path.is_some() {
            let list = self.args.list_path.clone().unwrap();
            path.push(format!("{}.{}", &list, ext.as_ref()));
            match chk {
                PathExitCondition::Exists => {
                    if path.exists() {
                        self.exit(ExitCode::FileExists(path));
                    }
                },
                PathExitCondition::NotExists => {
                    if !path.exists() {
                        self.v_print(
                            format!(
                                "File at \"{}\" does not exist",
                                &path.to_str().unwrap()
                            )
                        );
                        self.exit(ExitCode::FileDoesNotExist(path));
                    }
                },
                PathExitCondition::Ignore => {},
            }
        } else {
            self.exit(ExitCode::NoListName);
        }
    }
}
