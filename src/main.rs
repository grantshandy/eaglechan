use std::io;

use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

mod pagedata;

use pagedata::*;
use sqlx::SqlitePool;

const IP: &'static str = "127.0.0.1";
const PORT: u16 = 8080;

const DB_FILENAME: &'static str = "sqlite:data.db";

const INDEX: &'static str = include_str!("index.hbs");
const CSS: &'static str = include_str!("styles.css");

#[derive(Debug)]
struct AppState {
    pub template_registry: Handlebars<'static>,
    pub database: SqlitePool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    println!("starting HTTP server at http://{IP}:{PORT}/");

    let database = SqlitePool::connect(DB_FILENAME)
        .await
        .expect("Couldn't connect to database file.");

    HttpServer::new(move || {
        let app_data = web::Data::new(AppState {
            template_registry: generate_template_registry(),
            database: database.clone(),
        });

        App::new().app_data(app_data).service(index).service(css)
    })
    .bind((IP, PORT))?
    .run()
    .await
}

fn generate_template_registry() -> Handlebars<'static> {
    let mut template_registry = Handlebars::new();

    template_registry
        .register_template_string("index", INDEX)
        .unwrap();

    template_registry
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let page = data
        .template_registry
        .render("index", &PageState::generate(&data.database).await)
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(page)
}

#[get("/styles.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}
