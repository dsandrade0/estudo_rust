use actix_web::{Responder, web};
use serde_json::{Map, Value};
use super::utils::return_state;

pub async fn get() -> impl Responder {

    web::Json(return_state())
}