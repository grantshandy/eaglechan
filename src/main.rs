use std::io;

use actix_web::{
    get,
    http::header::ContentType,
    web::{Data, Path},
    App, HttpResponse, HttpServer,
};
use handlebars::Handlebars;
use serde::Serialize;
use sqlx::SqlitePool;

const IP: &'static str = "127.0.0.1";
const PORT: u16 = 8080;

const DB_FILENAME: &'static str = "sqlite:data.db";

const INDEX: &'static str = include_str!("templates/index.hbs");
const POST: &'static str = include_str!("templates/post.hbs");
const CSS: &'static str = include_str!("css/style.css");

#[derive(Debug)]
struct AppState {
    pub template_registry: Handlebars<'static>,
    pub database: SqlitePool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let database = SqlitePool::connect(DB_FILENAME)
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
            .service(index)
            .service(css)
            .service(post)
    })
    .bind((IP, PORT))?
    .run()
    .await
}

#[get("/")]
async fn index(data: Data<AppState>) -> HttpResponse {
    let posts: Vec<Post> = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&data.database)
        .await
        .unwrap();
    let pagestate = IndexPageState { posts };
    let page = data.template_registry.render("index", &pagestate).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(page)
}

#[get("/post-{id}")]
async fn post(id: Path<String>, data: Data<AppState>) -> HttpResponse {
    let id: i32 = id.parse().unwrap();

    let post: Post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = ? LIMIT 1", id)
        .fetch_one(&data.database)
        .await
        .unwrap();
    
    let pagestate = PostPageState { post, };
    let page = data.template_registry.render("post", &pagestate).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(page)
}

#[get("/styles.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

#[derive(Serialize)]
struct IndexPageState {
    posts: Vec<Post>,
}

#[derive(Serialize)]
struct PostPageState {
    post: Post
}

#[derive(Serialize, Debug)]
pub struct Post {
    id: i64,
    title: String,
    content: String,
    author: String,
}

fn generate_template_registry() -> Handlebars<'static> {
    let mut template_registry = Handlebars::new();

    template_registry
        .register_template_string("index", INDEX)
        .unwrap();
    
    template_registry
        .register_template_string("post", POST)
        .unwrap();

    return template_registry;
}
