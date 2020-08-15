use std::collections::HashMap;
use std::fs::File;
use actix_web::{web, HttpServer, App, HttpResponse, get};
use rand::seq::IteratorRandom;
use actix_web::http::header::LOCATION;
use actix_web::http::HeaderValue;
use actix_web::web::Data;
use actix_web::middleware::Logger;


fn get_json(json_path: &str) -> anyhow::Result<HashMap<String, String>> {
    let file = File::open(json_path)?;

    Ok(serde_json::from_reader(file)?)
}

#[derive(Clone)]
struct Urls (HashMap<String, String>);

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let urls = Urls(get_json("./urls.json")?);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(random)
            .service(choose)
            .data(urls.clone())
    })
        .bind("localhost:5001")?
        .run()
        .await?;

    Ok(())
}

#[get("/")]
async fn random(urls: Data<Urls>) -> HttpResponse {
    let redir = urls.as_ref().0.values().choose(&mut rand::thread_rng()).unwrap();

    HttpResponse::TemporaryRedirect()
        .set_header(LOCATION, HeaderValue::from_str(redir).unwrap())
        .finish()
}

#[get("/{id}")]
async fn choose(id: web::Path<String>, urls: Data<Urls>) -> actix_web::Result<HttpResponse> {
    match urls.as_ref().0.get(id.as_str()) {
        Some(url) => {
            Ok(HttpResponse::TemporaryRedirect()
                .set_header(LOCATION, HeaderValue::from_str(url).unwrap())
                .finish())
        },
        None => Err(HttpResponse::NotFound().into())
    }
}