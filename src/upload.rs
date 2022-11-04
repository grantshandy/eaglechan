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

#[post("/upload_thread")]
pub async fn upload_thread(
    req: HttpRequest,
    data: Data<AppState>,
    form: Form<PostForm>,
) -> HttpResponse {
    let mut resp = HttpResponse::SeeOther();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let thread_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_uppercase();

    let created: NaiveDateTime = Utc::now().naive_local();
    let last_updated = created.clone();

    sqlx::query!(
        "INSERT INTO threads ( thread_id, user_id, created, last_updated, title, content ) VALUES ( ?, ?, ?, ?, ?, ? )",
        thread_id,
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

#[post("/thread_comment_{thread_id}")]
pub async fn upload_comment(
    req: HttpRequest,
    data: Data<AppState>,
    thread_id: Path<String>,
    form: Form<CommentForm>,
) -> HttpResponse {
    let mut resp = HttpResponse::SeeOther();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let created: NaiveDateTime = Utc::now().naive_local();
    let thread_id = thread_id.to_string();

    // insert comment
    sqlx::query!(
        "INSERT INTO comments ( user_id, thread_id, content, created ) VALUES ( ?, ?, ?, ? )",
        user_id,
        thread_id,
        form.content,
        created,
    )
    .execute(&data.database)
    .await
    .expect("couldn't insert comment into database");

    // update last updated
    sqlx::query!(
        "UPDATE threads SET last_updated = ? WHERE thread_id = ?",
        created,
        thread_id
    )
    .execute(&data.database)
    .await
    .expect("failed to update thread");

    resp.insert_header((
        header::LOCATION,
        HeaderValue::from_str(&format!("/thread/{thread_id}")).expect("invalid header value"),
    ));

    return resp.finish();
}
