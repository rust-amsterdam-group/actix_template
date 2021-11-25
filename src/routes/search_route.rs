use actix_web::web::Data;
use actix_web::{get, web, Responder};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchRequest {
    query: String,
}

#[get("/search")]
pub async fn search(params: web::Query<SearchRequest>, client: Data<Client>) -> impl Responder {
    client
        .get(format!("https://en.wikipedia.org/wiki/{}", params.query))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_or("oh snap!", |_| "success")
}
