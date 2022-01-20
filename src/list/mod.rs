use chrono::DateTime;
use chrono::Local;
use crate::enums::ExitCode;
use crate::enums::PrintWhich;
use crate::enums::ItemType;
use crate::item::Item;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str as from_json_string;
use serde_json::to_string as to_json_string;
use std::ops::Add;
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
    pub fn print(&mut self, content: &mut String, print_which: &PrintWhich) {
        let created = self.created.format("%m/%d/%Y %H:%M:%S");
        let updated = self.last_updated.format("%m/%d/%Y %H:%M:%S");
        content.push_str(&format!("Created On: {}", created));
        content.push_str(&format!("\nLast Edit : {}", updated));
        let mut level = 0;
        let mut index = 1;
        if self.items.len().eq(&0) {
            content.push_str("\nThere are no items in this list");
            return;
        }
        for item in self.items.iter() {
            item.printable(content, &mut index, &mut level, print_which);
            index = index.add(1);
        }
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
    pub fn alter_check_at(&mut self, checked: bool, indices: &mut Vec<i32>) {
        let list_item_index = indices.pop().unwrap();
        let mut iter_c = 1;
        for act_item in self.items.iter_mut() {
            if iter_c.eq(&list_item_index) {
                act_item.alter_check(checked, indices);
                self.last_updated = Local::now();
                break;
            }
            iter_c = iter_c + 1;
        }
    }
    pub fn add_item(&mut self, item_type: ItemType, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        if indices.len().eq(&0) {
            self.items.push(Item::new(item_type, message));
            self.last_updated = Local::now();
        } else {
            let list_item_index = indices.pop().unwrap();
            let mut iter_c = 1;
            for act_item in self.items.iter_mut() {
                if iter_c.eq(&list_item_index) {
                    act_item.add_item(item_type, indices, message);
                    self.last_updated = Local::now();
                    break;
                }
                iter_c = iter_c + 1;
            }
        }
    }
    pub fn edit_at(&mut self, indices: &mut Vec<i32>, message: impl AsRef<str>) {
        let list_item_index = indices.pop().unwrap();
        let mut iter_c = 1;
        for act_item in self.items.iter_mut() {
            if iter_c.eq(&list_item_index) {
                act_item.edit_at(indices, message);
                self.last_updated = Local::now();
                break;
            }
            iter_c = iter_c + 1;
        }
    }
    pub fn remove_at(&mut self, indices: &mut Vec<i32>) -> Option<Item> {
        if indices.len().eq(&1) {
            let item = self.items.remove(
                (indices.get(0).unwrap().to_owned() as usize) - 1
            );
            self.last_updated = Local::now();
            return Some(item);
        } else {
            let list_item_index = indices.pop().unwrap();
            for iter_loc in 1..self.items.len() + 1 {
                if iter_loc.eq(&(list_item_index as usize)) {
                    let act_item = self.items.get_mut(iter_loc - 1).unwrap();
                    let item = act_item.remove_at(indices);
                    self.last_updated = Local::now();
                    return item;
                }
            }
            return None;
        }
    }
    fn put_item_at(&mut self, out_loc: &mut Vec<i32>, item: Item) {
        if out_loc.len().eq(&0) {
            self.items.push(item);
            self.last_updated = Local::now();
            return;
        } else {
            let index = out_loc.pop().unwrap();
            for iter_loc in 1..self.items.len() + 1 {
                if iter_loc.eq(&(index as usize)) {
                    let curr_item = self.items.get_mut(iter_loc - 1).unwrap();
                    curr_item.put_item_at(out_loc, item);
                    self.last_updated = Local::now();
                    return;
                }
            }
        }
    }
    pub fn move_from_to(&mut self, in_loc: &mut Vec<i32>, out_loc: &mut Vec<i32>) {
        let rem_item = match self.remove_at(&mut in_loc.clone()) {
            Some(item) => item,
            None => return,
        };
        self.put_item_at(out_loc, rem_item);
    }
}
