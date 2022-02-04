use chrono::Local;
use crate::container::Container;
use crate::enums::ItemStatus;
use crate::enums::ItemType;
use crate::item::Item;
use crate::list::List;
use std::borrow::BorrowMut;
pub trait ItemHolder {
    fn update_date(&mut self);
    fn borrow_items_mut(&mut self) -> &mut Vec<Item>;
}
impl ItemHolder for Item {
    fn update_date(&mut self) {
        self.last_updated = Local::now();
    }
    fn borrow_items_mut(&mut self) -> &mut Vec<Item> {
        self.sub_items.borrow_mut()
    }
}
impl ItemHolder for List {
    fn update_date(&mut self) {
        self.last_updated = Local::now();
    }
    fn borrow_items_mut(&mut self) -> &mut Vec<Item> {
        self.items.borrow_mut()
    }
}
impl ItemHolder for Container {
    fn update_date(&mut self) {
        self.list.last_updated = Local::now();
    }
    fn borrow_items_mut(&mut self) -> &mut Vec<Item> {
        self.list.items.borrow_mut()
    }
}
pub enum ItemAction {
    AlterStatus(ItemStatus),
    AlterHidden(bool),
    ToggleHidden,
    Add(ItemType, String),
    CycleStatus,
    Edit(String),
    Remove,
    Put(Item),
    ToggleType,
}
impl ItemAction {
    fn to_int(&self) -> i8 {
        match self {
            Self::AlterStatus(_) => 1,
            Self::AlterHidden(_) => 2,
            Self::Add(_, _) => 3,
            Self::Edit(_) => 4,
            Self::Remove => 5,
            Self::Put(_) => 6,
            Self::CycleStatus => 7,
            Self::ToggleHidden => 8,
            Self::ToggleType => 9,
        }
    }
    fn dirty_eq(&self, rhs: &Self) -> bool {
        self.to_int().eq(&rhs.to_int())
    }
}
trait ActOnItem {
    fn act_on_item(&mut self, indices: &mut Vec<usize>, action: ItemAction) -> Option<Item>;
}
impl ActOnItem for Item {
    fn act_on_item(&mut self, indices: &mut Vec<usize>, action: ItemAction) -> Option<Item> {
        if indices.len() == 0 {
            match action {
                ItemAction::AlterStatus(status) => {
                    self.status = status;
                },
                ItemAction::AlterHidden(hidden) => {
                    self.hidden = hidden;
                },
                ItemAction::Add(item_type, message) => {
                    self.sub_items.push(Self::new(item_type, message));
                },
                ItemAction::Edit(msg) => {
                    self.text = msg;
                },
                ItemAction::Remove => {},
                ItemAction::Put(item) => {
                    self.sub_items.push(item);
                },
                ItemAction::CycleStatus => {
                    let next_status = match self.status {
                        ItemStatus::Complete => ItemStatus::Disabled,
                        ItemStatus::Disabled => ItemStatus::Incomplete,
                        ItemStatus::Incomplete => ItemStatus::Complete,
                    };
                    self.status = next_status;
                },
                ItemAction::ToggleType => {
                    let next_type = match self.item_type {
                        ItemType::Todo => ItemType::Note,
                        ItemType::Note => ItemType::Todo,
                    };
                    self.item_type = next_type;
                },
                ItemAction::ToggleHidden => {
                    self.hidden = !self.hidden;
                },
            }
            self.update_date();
            return None;
        } else {
            self.update_date();
            return self.act_on_item_at(indices, action);
        }
    }
}
pub trait ItemActor {
    fn act_on_item_at(
        &mut self, indices: &mut Vec<usize>, action: ItemAction
    ) -> Option<Item>;
}
impl<Holder> ItemActor for Holder
where
    Holder: ItemHolder
{
    fn act_on_item_at(
        &mut self, indices: &mut Vec<usize>, action: ItemAction
    ) -> Option<Item> {
        if indices.len() == 0 {
            match action {
                ItemAction::Put(item) => {
                    self.update_date();
                    let items = self.borrow_items_mut();
                    items.push(item);
                    return None;
                },
                ItemAction::Add(item_type, message) => {
                    self.update_date();
                    let items = self.borrow_items_mut();
                    items.push(Item::new(item_type, message));
                    return None;
                },
                _ => return None,
            }
        } else if action.dirty_eq(&ItemAction::Remove) && indices.len() == 1 {
            self.update_date();
            let items = self.borrow_items_mut();
            let remove_index = indices.pop().unwrap();
            let mut item = items.remove(remove_index - 1);
            item.last_updated = Local::now();
            return Some(item);
        }
        let items = self.borrow_items_mut();
        let item_index = indices.pop().unwrap();
        for i in 1..(items.len() + 1) {
            if i == item_index {
                let item = items.get_mut(i - 1).unwrap();
                let out_item = item.act_on_item(indices, action);
                self.update_date();
                return out_item;
            }
        }
        return None;
    }
}
