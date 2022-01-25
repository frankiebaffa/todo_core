use chrono::Local;
use crate::container::Container;
use crate::enums::ItemStatus;
use crate::enums::ItemType;
use crate::item::Item;
use crate::list::List;
use std::borrow::BorrowMut;
pub trait ItemHolder {
    fn borrow_items_mut(&mut self) -> &mut Vec<Item>;
}
impl ItemHolder for Item {
    fn borrow_items_mut(&mut self) -> &mut Vec<Item> {
        self.sub_items.borrow_mut()
    }
}
impl ItemHolder for List {
    fn borrow_items_mut(&mut self) -> &mut Vec<Item> {
        self.items.borrow_mut()
    }
}
impl ItemHolder for Container {
    fn borrow_items_mut(&mut self) -> &mut Vec<Item> {
        self.list.items.borrow_mut()
    }
}
pub enum ItemAction {
    AlterStatus(ItemStatus),
    AlterHidden(bool),
    Add(ItemType, String),
    Edit(String),
    Remove,
    Put(Item),
}
impl ItemAction {
    fn to_int(&self) -> i8 {
        match self {
            ItemAction::AlterStatus(_) => 1,
            ItemAction::AlterHidden(_) => 2,
            ItemAction::Add(_, _) => 3,
            ItemAction::Edit(_) => 4,
            ItemAction::Remove => 5,
            ItemAction::Put(_) => 6,
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
            }
            self.last_updated = Local::now();
            return None;
        } else {
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
                    let items = self.borrow_items_mut();
                    items.push(item);
                    return None;
                },
                ItemAction::Add(item_type, message) => {
                    let items = self.borrow_items_mut();
                    items.push(Item::new(item_type, message));
                    return None;
                },
                _ => return None,
            }
        } else if action.dirty_eq(&ItemAction::Remove) && indices.len() == 1 {
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
                return item.act_on_item(indices, action);
            }
        }
        return None;
    }
}
