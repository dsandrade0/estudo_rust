use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(res) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}", res);
            }
            None => println!("item {} was not found", title)
        }
    }
}