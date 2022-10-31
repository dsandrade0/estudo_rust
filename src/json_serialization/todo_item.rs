use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub status: String
}