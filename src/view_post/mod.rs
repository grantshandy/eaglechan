use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, HttpResponse,
};
use serde::Serialize;

use crate::{AppState, Comment, Post};

pub const TEMPLATE: &'static str = include_str!("view_post.hbs");

#[derive(Serialize)]
struct PageState {
    user_id: String,
    post_id: String,
    post: Post,
    comments: Vec<Comment>,
}

#[get("/post-{id}")]
pub async fn get_post(
    req: HttpRequest,
    data: Data<AppState>,
    post_id: Path<String>,
) -> HttpResponse {
    let post_id = post_id.to_string();

    let mut resp = HttpResponse::Ok();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let post: Post = sqlx::query_as!(
        Post,
        "SELECT * FROM posts WHERE post_id = ? LIMIT 1",
        post_id
    )
    .fetch_one(&data.database)
    .await
    .expect("no post found");

    let comments: Vec<Comment> =
        sqlx::query_as!(Comment, "SELECT * FROM comments WHERE post_id = ?", post_id)
            .fetch_all(&data.database)
            .await
            .expect("failed to get comments");

    let page = data
        .template_registry
        .render(
            "view_post",
            &PageState {
                user_id,
                post_id,
                post,
                comments,
            },
        )
        .unwrap();

    return resp.body(page);
}
