use std::fmt::format;
use actix_web::{HttpResponse, web};
use actix_web::web::to;
use serde_json::{Map, Value};
use crate::json_serialization::todo_item::TodoItem;
use crate::process::process_input;
use crate::state::read_file;
use crate::todo::todo_factory;
use crate::views::todo::utils::return_state;

pub async fn edit(todo_item: web::Json<TodoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let title_reference: &String = &todo_item.title.clone();
    let status: String;

    match &state.get(title_reference) {
        Some(res) => {
            status = res.to_string().replace("\"", "");
        },
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", title_reference))
        }
    }

    if &status == &todo_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    match todo_factory(&status, title_reference) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accept", status)),
        Ok(_item) => process_input(_item, "edit", &state)
    }

    return HttpResponse::Ok().json(return_state());
}