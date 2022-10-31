use actix_web::HttpRequest;
use serde_json::{Map, Value};
use crate::todo;
use crate::state::read_file;
use crate::process;
use crate::process::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");

    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone();

    let item = todo::todo_factory("pending", title.as_str()).expect("create ");

    process_input(item, "create", &state);
    return format!("{} created", title_reference)
}