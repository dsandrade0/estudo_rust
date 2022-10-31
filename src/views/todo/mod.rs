pub mod create;
pub mod edit;
pub mod get;
pub mod utils;
use actix_web::web;
use crate::views::path::Path;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path{prefix: String::from("/item")};

    app.route(
        &base_path.define(String::from("/create/{title}")),
        web::post().to(create::create));
    app.route(
        &base_path.define(String::from("")),
        web::get().to(get::get));
}