use serde_json::{Map, Value};
use crate::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("{} is being deleted", title);
    }
}