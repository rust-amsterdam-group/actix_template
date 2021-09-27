use actix_web::{App, get, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = match std::env::var("PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_e) => "127.0.0.1:8080".into()
    };
    HttpServer::new(|| App::new().service(index))
        .bind(&address)?
        .run()
        .await
}
