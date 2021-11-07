pub mod watch;
pub mod facility;
pub mod target;
pub mod associated;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use serde::Deserialize;


#[derive(Deserialize)]
struct Info {
    username: String,
}

pub type WrappedResult<T> = Result<T, Box<dyn std::error::Error>>;

/// extract `Info` using serde
async fn handle_associated_token_account(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/associated-token-account", web::post().to(handle_associated_token_account))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}