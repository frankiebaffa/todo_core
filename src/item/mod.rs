use {
    chrono::{
        DateTime,
        Local,
    },
    crate::{
        enums::{
            PrintWhich,
            ItemStatus,
            ItemType,
        },
        utils::styler,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        io::Error as IOError,
        ops::Add,
    },
};
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub item_type: ItemType,
    pub status: ItemStatus,
    pub text: String,
    pub sub_items: Vec<Item>,
    pub created: DateTime<Local>,
    pub last_updated: DateTime<Local>,
    pub hidden: bool,
}
impl Item {
    pub fn new(item_type: ItemType, text: impl AsRef<str>) -> Self {
        let txt = text.as_ref().to_string();
        Self {
            item_type,
            status: ItemStatus::Incomplete,
            text: txt,
            sub_items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
            hidden: false,
        }
    }
    fn has_complete(&self) -> bool {
        if self.item_type.eq(&ItemType::Todo) && self.status.eq(&ItemStatus::Complete) {
            return true;
        }
        for item in self.sub_items.iter() {
            if item.has_complete() {
                return true;
            }
        }
        return false;
    }
    fn has_incomplete(&self) -> bool {
        if self.item_type.eq(&ItemType::Todo) && self.status.eq(&ItemStatus::Incomplete) {
            return true;
        }
        for item in self.sub_items.iter() {
            if item.has_incomplete() {
                return true;
            }
        }
        return false;
    }
    pub fn get_highest_num(&self, mut cmp: usize) {
        let highest_num = self.sub_items.len() + 1;
        if highest_num > cmp {
            cmp = highest_num;
        }
        for item in self.sub_items.iter() {
            item.get_highest_num(cmp);
        }
    }
    fn get_spacing(index: usize, spacing: usize) -> String {
        let mut s = String::new();
        let i_len = index.to_string().len();
        for _ in 0..(spacing - i_len) {
            s.push_str(" ");
        }
        return s;
    }
    pub fn printable(
        &self, output: &mut String, index: &mut usize, level: &mut usize,
        print_which: &PrintWhich, plain: bool, spacing: usize,
        max_level: Option<usize>, parent_is_hidden: bool, display_hidden: bool,
    ) -> Result<(), IOError> {
        match print_which {
            PrintWhich::All => {},
            PrintWhich::Complete => {
                if !self.has_complete() {
                    return Ok(());
                }
            },
            PrintWhich::Incomplete => {
                if !self.has_incomplete() {
                    return Ok(());
                }
            },
        }
        let mut indent = String::new();
        for _ in 0..level.clone() {
            for _ in 0..spacing {
                indent.push_str(" ");
            }
            indent.push_str("      ");
        }
        let show_this = (!self.hidden && !parent_is_hidden) || display_hidden;
        if show_this {
            match self.item_type {
                ItemType::Todo => {
                    if !plain {
                        let status = format!(
                            "{}. {}[{}]",
                            index,
                            Self::get_spacing(*index, spacing),
                            self.status.symbol(),
                        );
                        let status_line = styler::bold(
                            format!("\n{}{} ", indent, status)
                        );
                        match self.status.clone() {
                            ItemStatus::Complete => {
                                output.push_str(&styler::success(status_line));
                            },
                            ItemStatus::Disabled => {
                                output.push_str(&styler::warning(status_line));
                            },
                            ItemStatus::Incomplete => {
                                output.push_str(&styler::danger(status_line));
                            },
                        }
                        output.push_str(&format!("{}", self.text));
                    } else {
                        output.push_str(&format!(
                            "\n{}{}. {}[{}] {}",
                            indent,
                            index,
                            Self::get_spacing(*index, spacing),
                            self.status.symbol(),
                            self.text
                        ));
                    }
                },
                ItemType::Note => {
                    if !plain {
                        let status = format!("\n{}{}. ", indent, index);
                        let status_line = styler::bold(status);
                        output.push_str(&styler::info(status_line));
                        output.push_str(&format!(
                            "{}    {}",
                            Self::get_spacing(*index, spacing),
                            self.text
                        ));
                    } else {
                        output.push_str(&format!(
                            "\n{}{}. {}    {}",
                            indent,
                            index,
                            Self::get_spacing(*index, spacing),
                            self.text
                        ));
                    }
                },
            }
        }
        match max_level {
            Some(max) => {
                if (*level).eq(&(max - 1)) {
                    return Ok(());
                }
            },
            None => {},
        }
        let mut sub_index = 1;
        for sub in self.sub_items.iter() {
            sub.printable(
                output, &mut sub_index, &mut (level.add(1)), print_which,
                plain, spacing, max_level, !show_this, display_hidden,
            )?;
            sub_index = sub_index + 1;
        }
        Ok(())
    }
    pub fn count_complete(&self) -> usize {
        let mut counter = 0;
        if self.item_type.eq(&ItemType::Todo) && self.status.eq(&ItemStatus::Complete) {
            counter = counter + 1;
        }
        for sub in self.sub_items.iter() {
            counter = sub.count_complete();
        }
        counter
    }
    pub fn count_incomplete(&self) -> usize {
        let mut counter = 0;
        if self.item_type.eq(&ItemType::Todo) && !self.status.eq(&ItemStatus::Incomplete) {
            counter = counter + 1;
        }
        for sub in self.sub_items.iter() {
            counter = sub.count_incomplete();
        }
        counter
    }
}
