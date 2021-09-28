use std::sync::Arc;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use reqwest::Client;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = match std::env::var("PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_e) => "127.0.0.1:8080".into()
    };
    HttpServer::new(||
        App::new()
            .service(routes::index)
            .service(routes::search)
            .app_data(Data::from(Arc::new(Client::new())))
    )
        .bind(&address)?
        .run()
        .await
}