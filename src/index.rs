use actix_web::{get, http::header::ContentType, web::Data, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::{post::Post, AppState};

pub const INDEX_TEMPLATE: &'static str = include_str!("templates/index.hbs");

#[derive(Serialize)]
pub struct IndexPageState {
    posts: Vec<Post>,
}

#[get("/")]
pub async fn get_index(data: Data<AppState>) -> HttpResponse {
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
