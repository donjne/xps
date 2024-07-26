use actix_web::{web, App, HttpServer};
use crate::auth::middleware::AuthMiddleware;

pub async fn run_server() {
    HttpServer::new(move|| {
    App::new()
    .route("/register", web::post().to(register))
    .route("/login", web::post().to(login))
    .wrap(AuthMiddleware)
})
.bind("127.0.0.1:8080")
.expect("Can not bind to port 8080")
.run().await.unwrap();
}