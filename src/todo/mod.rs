pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn todo_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        let pending_item: Pending = Pending::new(item_title);
        return Ok(ItemTypes::Pending(pending_item))
    }

    if item_type == "done" {
        let done_item = Done::new(item_title);
        return Ok(ItemTypes::Done(done_item))
    }

    Err("this is not accept")
}