use std::io;

use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

const IP: &'static str = "127.0.0.1";
const PORT: u16 = 8080;

const INDEX: &'static str = include_str!("index.hbs");
const CSS: &'static str = include_str!("styles.css");

#[derive(Serialize, Deserialize)]
struct PageState {
    time: String,
}

impl PageState {
    pub fn generate() -> Self {
        Self {
            time: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Clone, Debug)]
struct AppState {
    pub template_registry: Handlebars<'static>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    println!("starting HTTP server at http://{IP}:{PORT}/");

    HttpServer::new(|| {
        let mut template_registry = Handlebars::new();

        template_registry
            .register_template_string("index", INDEX)
            .unwrap();

        let app_data = web::Data::new(AppState { template_registry });

        App::new().app_data(app_data).service(index).service(css)
    })
    .bind((IP, PORT))?
    .run()
    .await
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let page = data
        .template_registry
        .render("index", &PageState::generate())
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(page)
}

#[get("/styles.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(CSS)
}
