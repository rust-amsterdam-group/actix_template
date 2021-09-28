use actix_web::{get, Responder, web};
use actix_web::web::Data;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchRequest {
    query: String,
}

#[get("/search")]
pub async fn search(info: web::Query<SearchRequest>, client: Data<Client>) -> impl Responder {
    let response = client.get(format!("https://www.google.com/search?q={}", info.query))
        .send()
        .await
        .and_then(|r| r.error_for_status());

    return match response {
        Ok(_) => "success",
        Err(_) => "oh snap"
    };
}