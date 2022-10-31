use serde_json::{Map, Value};
use crate::json_serialization::todo_items::TodoItems;
use crate::state::read_file;
use crate::todo::{ItemTypes, todo_factory};

pub fn return_state() -> TodoItems {
    let state: Map<String, Value> = read_file("./state.json");
    let mut arr_buffer = Vec::new();

    for (k,v) in state {
        let item_type: String = String::from(v.as_str().unwrap());
        let item: ItemTypes = todo_factory(&item_type, &k).unwrap();
        arr_buffer.push(item)
    }
    TodoItems::new(arr_buffer)
}