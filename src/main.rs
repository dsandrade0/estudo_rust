use actix_web::{App, HttpRequest, HttpServer, Responder, web};
mod state;
mod todo;
mod views;
mod process;
mod json_serialization;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory);
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
