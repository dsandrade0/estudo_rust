use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::{Error, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::body::{BodySize, MessageBody};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::{Bytes, Json};
use futures::future::{ready, Ready};
use serde_derive::Serialize;

use crate::todo::ItemTypes;
use crate::todo::structs::base::Base;

#[derive(Serialize)]
pub struct TodoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}

impl TodoItems {
    pub fn new(input: Vec<ItemTypes>) -> TodoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input {
            match item {
                ItemTypes::Pending(i) => pending_array_buffer.push(i.super_struct),
                ItemTypes::Done(i) => done_array_buffer.push(i.super_struct)
            }
        }

        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        return TodoItems {
            pending_items: pending_array_buffer,
            pending_item_count: pending_count,
            done_items: done_array_buffer,
            done_item_count: done_count
        }
    }
}