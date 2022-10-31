use actix_web::{App, HttpRequest, HttpServer, Responder, web};
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory);
        return app
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
