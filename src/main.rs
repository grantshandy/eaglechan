use std::{fs, io, path::Path};

use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use sqlx::SqlitePool;

mod index;
mod post;

use index::{get_index, INDEX_TEMPLATE};
use post::{get_post, POST_TEMPLATE};

const DATABASE_TEMPLATE: &[u8] = include_bytes!("template.db");
const CSS: &'static str = include_str!("css/style.css");

// WEB ROUTES
// GET "/" -> index::get_index
// GET "/post_{id}" -> post::get_post

#[derive(argh::FromArgs)]
/// An anonymous forum.
struct Args {
    /// what database to use
    #[argh(option, default = "\"eaglechan.db\".to_string()")]
    database: String,
    /// what port to serve on
    #[argh(option, default = "8080")]
    port: u16,
    /// what ip to serve on
    #[argh(option, default = "\"127.0.0.1\".to_string()")]
    ip: String,
}

#[derive(Debug)]
pub struct AppState {
    pub template_registry: Handlebars<'static>,
    pub database: SqlitePool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let args: Args = argh::from_env();
    generate_template_database(&args.database);
    
    println!("connecting to database {}", &args.database);

    let database = SqlitePool::connect(&format!("sqlite:{}", &args.database))
        .await
        .expect("Couldn't connect to database file.");

    println!("starting HTTP server at http://{}:{}/", &args.ip, &args.port);

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
    .bind((args.ip, args.port))?
    .run()
    .await
}

#[get("/styles.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

fn generate_template_database(db_filename: &str) {
    if !Path::new(db_filename)
        .try_exists()
        .expect("couldn't access filesystem")
    {
        fs::write(db_filename, DATABASE_TEMPLATE)
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
