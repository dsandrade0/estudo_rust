use serde_json::{json, Map, Value};
use super::super::super::super::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file("./state.json", state);
        println!("{} is being set to done", title);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file("./state.json", state);
        println!("{} is being set to pending", title);
    }
}