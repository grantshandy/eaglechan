use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{Data, Path},
    HttpResponse,
};
use serde::Serialize;

use crate::AppState;

pub const POST_TEMPLATE: &'static str = include_str!("templates/post.hbs");

#[derive(Serialize)]
pub struct PostPageState {
    post: Post,
}

#[derive(Serialize, Debug)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
}

#[get("/post_{id}")]
pub async fn get_post(id: Path<String>, data: Data<AppState>) -> HttpResponse {
    let post_id: i32 = id.parse().unwrap();

    let post: Post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE post_id = ? LIMIT 1", post_id)
        .fetch_one(&data.database)
        .await
        .unwrap();

    let pagestate = PostPageState { post };
    let page = data.template_registry.render("post", &pagestate).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(page)
}

#[post("/create-post")]
pub async fn create_post() -> HttpResponse {
    HttpResponse::Ok().finish()
}
