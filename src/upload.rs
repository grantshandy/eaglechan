use actix_web::{
    http::header::{self, HeaderValue},
    post,
    web::{Data, Form, Path},
    HttpRequest, HttpResponse,
};
use chrono::{NaiveDateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct PostForm {
    pub title: String,
    pub content: String,
}

#[post("/upload")]
pub async fn upload_post(
    req: HttpRequest,
    data: Data<AppState>,
    form: Form<PostForm>,
) -> HttpResponse {
    let mut resp = HttpResponse::SeeOther();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let post_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>();

    let created: NaiveDateTime = Utc::now().naive_local();
    let last_updated = created.clone();

    sqlx::query!(
        "INSERT INTO posts ( post_id, user_id, created, last_updated, title, content ) VALUES ( ?, ?, ?, ?, ?, ? )",
        post_id,
        user_id,
        created,
        last_updated,
        form.title,
        form.content,
    )
    .execute(&data.database)
    .await
    .expect("failed to insert post into database");

    resp.insert_header((
        header::LOCATION,
        HeaderValue::from_str("/").expect("invalid header value"),
    ));

    return resp.finish();
}

#[derive(Deserialize)]
pub struct CommentForm {
    content: String,
}

#[post("/comment-{post_id}")]
pub async fn upload_comment(
    req: HttpRequest,
    data: Data<AppState>,
    post_id: Path<String>,
    form: Form<CommentForm>,
) -> HttpResponse {
    let mut resp = HttpResponse::SeeOther();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let created: NaiveDateTime = Utc::now().naive_local();
    let post_id = post_id.to_string();

    sqlx::query!(
        "INSERT INTO comments ( user_id, post_id, content, created ) VALUES ( ?, ?, ?, ? )",
        user_id,
        post_id,
        form.content,
        created,
    )
    .execute(&data.database)
    .await
    .expect("couldn't insert comment into database");

    resp.insert_header((
        header::LOCATION,
        HeaderValue::from_str(&format!("/post-{post_id}")).expect("invalid header value"),
    ));

    return resp.finish();
}
