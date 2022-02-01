use {
    chrono::{
        DateTime,
        Local,
    },
    crate::{
        traits::Terminal,
        enums::{
            ExitCode,
            PrintWhich,
        },
        item::Item,
    },
    crossterm::style::{
        Color,
        SetForegroundColor,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    serde_json::{
        from_str as from_json_string,
        to_string as to_json_string,
    },
    std::{
        io::Error as IOError,
        ops::Add,
    },
};
#[derive(Serialize, Deserialize)]
pub struct List {
    pub name: String,
    pub items: Vec<Item>,
    pub created: DateTime<Local>,
    pub last_updated: DateTime<Local>,
}
impl List {
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
        }
    }
    pub fn from_json(json: String) -> Result<Self, ExitCode> {
        let list = match from_json_string(&json) {
            Ok(list) => list,
            Err(e) => return Err(ExitCode::FailedToDeserialize(e)),
        };
        Ok(list)
    }
    pub fn to_json(&self) -> Result<String, ExitCode> {
        let json = match to_json_string(self) {
            Ok(json) => json,
            Err(e) => return Err(ExitCode::FailedToSerialize(e)),
        };
        Ok(json)
    }
    fn get_highest_num(&self) -> usize {
        let highest_num = self.items.len() + 1;
        for item in self.items.iter() {
            item.get_highest_num(highest_num);
        }
        highest_num
    }
    fn get_spacing_count(&self) -> usize {
        let highest_num = self.get_highest_num();
        return highest_num.to_string().len();
    }
    pub fn print(
        &mut self, ctx: &mut impl Terminal, print_which: &PrintWhich, plain: bool,
        max_level: Option<usize>, display_hidden: bool,
    ) -> Result<(), IOError> {
        let created = format!("{}", self.created.format("%m/%d/%Y %H:%M:%S"));
        let updated = format!("{}", self.last_updated.format("%m/%d/%Y %H:%M:%S"));
        if !plain {
            ctx.queue_cmd(SetForegroundColor(Color::Blue))?;
            ctx.write_str("Created On: ")?;
            ctx.queue_cmd(SetForegroundColor(Color::Cyan))?;
            ctx.write_str(format!("{}", created))?;
            ctx.queue_cmd(SetForegroundColor(Color::Blue))?;
            ctx.write_str("\nLast Edit : ")?;
            ctx.queue_cmd(SetForegroundColor(Color::Cyan))?;
            ctx.write_str(format!("{}", updated))?;
        } else {
            ctx.write_str(
                &format!(
                    "Created On: {}",
                    created,
                )
            )?;
            ctx.write_str(
                &format!(
                    "\nLast Edit : {}",
                    updated
                )
            )?;
        }
        let mut level = 0;
        let mut index = 1;
        if self.items.len().eq(&0) {
            ctx.write_str("\n There are no items in this list")?;
            return Ok(());
        }
        let spacing = self.get_spacing_count();
        for item in self.items.iter() {
            item.printable(
                ctx, &mut index, &mut level, print_which, plain, spacing,
                max_level, false, display_hidden,
            )?;
            index = index.add(1);
        }
        Ok(())
    }
    pub fn status(&mut self, content: &mut String, print_which: &PrintWhich) {
        match print_which {
            PrintWhich::All => {
                content.push_str(&format!("Items: {}", self.items.len()));
            },
            _ => {},
        }
        if self.items.len().eq(&0) {
            return;
        }
        let mut complete = 0;
        let mut incomplete = 0;
        for item in self.items.iter() {
            match print_which {
                PrintWhich::All => {
                    complete = complete + item.count_complete();
                    incomplete = incomplete + item.count_incomplete();
                },
                PrintWhich::Complete => {
                    complete = complete + item.count_complete();
                },
                PrintWhich::Incomplete => {
                    incomplete = incomplete + item.count_incomplete();
                },
            }
        }
        match print_which {
            PrintWhich::All => {
                content.push_str(&format!("\nComplete: {}", complete));
                content.push_str(&format!("\nIncomplete: {}", incomplete));
            },
            PrintWhich::Complete => {
                content.push_str(&format!("\nComplete: {}", complete));
            },
            PrintWhich::Incomplete => {
                content.push_str(&format!("\nIncomplete: {}", incomplete));
            },
        }
    }
}
