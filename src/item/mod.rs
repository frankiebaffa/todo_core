use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Add;
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub checked: bool,
    pub text: String,
    pub sub_items: Vec<Item>,
    pub created: DateTime<Local>,
    pub last_updated: DateTime<Local>,
}
impl Item {
    pub fn new(text: impl AsRef<str>) -> Self {
        let txt = text.as_ref().to_string();
        Self {
            checked: false,
            text: txt,
            sub_items: Vec::new(),
            created: Local::now(),
            last_updated: Local::now(),
        }
    }
    pub fn alter_check(&mut self, checked: bool, indices: &mut Vec<i32>) {
        if indices.len().eq(&0) {
            self.checked = checked;
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.alter_check(checked, indices);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn add_item(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        if indices.len().eq(&0) {
            self.sub_items.push(Self::new(message));
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.add_item(indices, message);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn edit(&mut self, message: impl AsRef<str>) {
        self.text = message.as_ref().to_string();
    }
    pub fn edit_at(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
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
    pub fn remove_at(&mut self, indices: &mut Vec<i32>) {
        if indices.len().eq(&1) {
            let index = indices.get(0).unwrap().to_string().parse::<usize>().unwrap();
            self.sub_items.remove(index-1);
            self.last_updated = Local::now();
        } else {
            let index = indices.pop().unwrap();
            let mut iter_c = 1;
            for item in self.sub_items.iter_mut() {
                if iter_c.eq(&index) {
                    item.remove_at(indices);
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn printable(&self, content: &mut String, index: &mut usize, level: &mut usize) {
        let mut indent = String::new();
        for _ in 0..level.clone() {
            indent.push_str("    ");
        }
        let chked = if self.checked {
            "[x]"
        } else {
            "[ ]"
        };
        content.push_str(&format!(concat!(
            "{}{}. {} {}\n"
        ), indent, index, chked, self.text));
        let mut sub_index = 1;
        for sub in self.sub_items.iter() {
            sub.printable(content, &mut sub_index, &mut (level.add(1)));
            sub_index = sub_index + 1;
        }
    }
}
