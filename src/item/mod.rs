use chrono::DateTime;
use chrono::Local;
use crate::color_scheme;
use crate::enums::PrintWhich;
use crate::enums::ItemStatus;
use crate::enums::ItemType;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Add;
#[derive(Serialize, Deserialize)]
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
    pub fn alter_check(&mut self, status: ItemStatus, indices: &mut Vec<usize>) {
        if indices.len().eq(&0) {
            self.status = status;
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.alter_check(status, indices);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn alter_hidden(&mut self, hidden: bool, indices: &mut Vec<usize>) {
        if indices.len().eq(&0) {
            self.hidden = hidden;
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.alter_hidden(hidden, indices);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn add_item(
        &mut self, item_type: ItemType, indices: &mut Vec<usize>,
        message: impl AsRef<str>
    ) {
        if indices.len().eq(&0) {
            self.sub_items.push(Self::new(item_type, message));
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.add_item(item_type, indices, message);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn edit(&mut self, message: impl AsRef<str>) {
        self.text = message.as_ref().to_string();
    }
    pub fn edit_at(&mut self, indices: &mut Vec<usize>, message: impl AsRef<str>) {
        if indices.len().eq(&0) {
            self.edit(message);
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.edit_at(indices, message);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn remove_at(&mut self, indices: &mut Vec<usize>) -> Option<Item> {
        if indices.len().eq(&1) {
            let index = indices.get(0).unwrap().to_string().parse::<usize>().unwrap();
            let item = self.sub_items.remove(index-1);
            self.last_updated = Local::now();
            return Some(item);
        } else {
            let index = indices.pop().unwrap();
            for iter_loc in 1..self.sub_items.len() + 1 {
                if iter_loc.eq(&index) {
                    let item = self.sub_items.get_mut(iter_loc).unwrap();
                    return item.remove_at(indices);
                }
            }
            return None;
        }
    }
    pub fn put_item_at(&mut self, indices: &mut Vec<usize>, item: Item) {
        if indices.len().eq(&0) {
            self.sub_items.push(item);
            self.last_updated = Local::now();
            return;
        } else {
            let index = indices.pop().unwrap().to_owned();
            for iter_loc in 1..self.sub_items.len() + 1 {
                if iter_loc.eq(&index) {
                    let sub_item = self.sub_items.get_mut(iter_loc).unwrap();
                    sub_item.put_item_at(indices, item);
                    return;
                }
            }
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
        &self, content: &mut String, index: &mut usize, level: &mut usize,
        print_which: &PrintWhich, plain: bool, spacing: usize,
        max_level: Option<usize>, parent_is_hidden: bool, display_hidden: bool,
    ) {
        match print_which {
            PrintWhich::All => {},
            PrintWhich::Complete => {
                if !self.has_complete() {
                    return;
                }
            },
            PrintWhich::Incomplete => {
                if !self.has_incomplete() {
                    return;
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
                        match self.status.clone() {
                            ItemStatus::Complete => {
                                content.push_str(
                                    &format!(
                                        "\n{}{} {}",
                                        indent,
                                        color_scheme::success(status),
                                        self.text
                                    )
                                );
                            },
                            ItemStatus::Disabled => {
                                content.push_str(
                                    &format!(
                                        "\n{}{} {}",
                                        indent,
                                        color_scheme::warning(status),
                                        self.text,
                                    )
                                );
                            },
                            ItemStatus::Incomplete => {
                                content.push_str(
                                    &format!(
                                        "\n{}{} {}",
                                        indent,
                                        color_scheme::danger(status),
                                        self.text,
                                    )
                                );
                            },
                        }
                    } else {
                        content.push_str(
                            &format!(
                                "\n{}{}. {}[{}] {}",
                                indent,
                                index,
                                Self::get_spacing(*index, spacing),
                                self.status.symbol(),
                                self.text
                            )
                        );
                    }
                },
                ItemType::Note => {
                    if !plain {
                        content.push_str(
                            &format!(
                                "\n{}{} {}    {}",
                                indent,
                                color_scheme::info(format!("{}.", index)),
                                Self::get_spacing(*index, spacing),
                                self.text
                            )
                        );
                    } else {
                        content.push_str(
                            &format!(
                                "\n{}{}. {}    {}",
                                indent,
                                index,
                                Self::get_spacing(*index, spacing),
                                self.text
                            )
                        );
                    }
                },
            }
        }
        match max_level {
            Some(max) => {
                if (*level).eq(&(max - 1)) {
                    return;
                }
            },
            None => {},
        }
        let mut sub_index = 1;
        for sub in self.sub_items.iter() {
            sub.printable(
                content, &mut sub_index, &mut (level.add(1)), print_which,
                plain, spacing, max_level, !show_this, display_hidden,
            );
            sub_index = sub_index + 1;
        }
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
