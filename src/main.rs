use std::{fs, io, path};

use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use sqlx::SqlitePool;

mod index;
mod post;

use index::{get_index, INDEX_TEMPLATE};
use post::{get_post, POST_TEMPLATE};

const IP: &'static str = "127.0.0.1";
const PORT: u16 = 8080;

const DATABASE_FILENAME: &'static str = "eaglechan.db";
const DATABASE_TEMPLATE: &[u8] = include_bytes!("template.db");

const CSS: &'static str = include_str!("css/style.css");

// WEB ROUTES
// GET "/" -> index::get_index
// GET "/post_{id}" -> post::get_post

#[derive(Debug)]
pub struct AppState {
    pub template_registry: Handlebars<'static>,
    pub database: SqlitePool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    gen_database_template();

    let database = SqlitePool::connect(&format!("sqlite:{}", DATABASE_FILENAME))
        .await
        .expect("Couldn't connect to database file.");

    println!("starting HTTP server at http://{IP}:{PORT}/");

    HttpServer::new(move || {
        let app_data = Data::new(AppState {
            template_registry: generate_template_registry(),
            database: database.clone(),
        });

        App::new()
            .app_data(app_data)
            .service(css)
            .service(get_index)
            .service(get_post)
    })
    .bind((IP, PORT))?
    .run()
    .await
}

#[get("/styles.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

fn gen_database_template() {
    if !path::Path::new(DATABASE_FILENAME)
        .try_exists()
        .expect("couldn't access filesystem")
    {
        fs::write(DATABASE_FILENAME, DATABASE_TEMPLATE)
            .expect("couldn't write database template to current directory");
    }
}

fn generate_template_registry() -> Handlebars<'static> {
    let mut template_registry = Handlebars::new();

    template_registry
        .register_template_string("index", INDEX_TEMPLATE)
        .unwrap();

    template_registry
        .register_template_string("post", POST_TEMPLATE)
        .unwrap();

    return template_registry;
}
