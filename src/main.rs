use std::{fs, io, path::Path};

use actix_web::{
    get,
    http::header::{self, HeaderValue},
    web::Data,
    App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
};
use chrono::NaiveDateTime;
use handlebars::Handlebars;
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;
use sqlx::SqlitePool;

mod index;
mod upload;
mod view_post;
mod write_post;

const DATABASE_TEMPLATE: &[u8] = include_bytes!("template.db");
const CSS: &'static str = include_str!("css/style.css");

// WEB ROUTES
// GET "/" -> index::get_index                                 DONE
// GET "/post-{post_id}" -> view_post::get_post                DONE
// GET "/write" -> write_post::get_write_post                  DONE
// POST "/upload" -> upload::upload_post                       DONE
// POST "/comment-{post_id}" upload::upload_comment            DONE

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

#[derive(Serialize, Debug)]
pub struct Post {
    pub post_id: String,
    pub user_id: String,
    pub created: NaiveDateTime,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct Comment {
    pub user_id: String,
    pub post_id: String,
    pub created: NaiveDateTime,
    pub content: String,
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

    println!(
        "starting HTTP server at http://{}:{}/",
        &args.ip, &args.port
    );

    HttpServer::new(move || {
        let app_data = Data::new(AppState {
            template_registry: generate_template_registry(),
            database: database.clone(),
        });

        App::new()
            .app_data(app_data)
            .service(css)
            .service(index::get_index)
            .service(view_post::get_post)
            .service(write_post::get_write_post)
            .service(upload::upload_post)
            .service(upload::upload_comment)
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
        .register_template_string("index", index::TEMPLATE)
        .unwrap();

    template_registry
        .register_template_string("view_post", view_post::TEMPLATE)
        .unwrap();

    template_registry
        .register_template_string("write_post", write_post::TEMPLATE)
        .unwrap();

    return template_registry;
}

/// returns your user id and a response that may contain a set cookie for the user.
pub async fn manage_cookies(
    req: &HttpRequest,
    data: &Data<AppState>,
    response: &mut HttpResponseBuilder,
) -> String {
    let (user_token, created): (String, bool) = match req.cookie("userToken") {
        Some(cookie) => (cookie.value().to_string(), false),
        None => {
            let user_id = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect::<String>();

            let user_token = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>();

            sqlx::query!(
                "INSERT INTO users ( user_token, user_id ) VALUES ( ?, ? )",
                user_token,
                user_id
            )
            .execute(&data.database)
            .await
            .expect("failed to insert new user_id and user_tokens into users");

            (user_token, true)
        }
    };

    let user_id: String =
        sqlx::query!("SELECT user_id FROM users WHERE user_token = ?", user_token)
            .fetch_one(&data.database)
            .await
            .expect("failed to get user_id from user_token in users")
            .user_id;

    if created {
        response.insert_header((
            header::SET_COOKIE,
            HeaderValue::from_str(&format!("userToken={user_token}"))
                .expect("invalud header value"),
        ));
    }

    return user_id;
}
