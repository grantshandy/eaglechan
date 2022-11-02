use actix_web::{
    http::header::{self, HeaderValue},
    post,
    web::{Data, Form},
    HttpRequest, HttpResponse,
};
use chrono::{NaiveDateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct UploadForm {
    pub title: String,
    pub content: String,
}

#[post("/upload")]
pub async fn upload_post(
    req: HttpRequest,
    data: Data<AppState>,
    form: Form<UploadForm>,
) -> HttpResponse {
    let mut resp = HttpResponse::SeeOther();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let post_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>();
    
    let created: NaiveDateTime = Utc::now().naive_local();

    sqlx::query!(
        "INSERT INTO posts ( post_id, user_id, created, title, content ) VALUES ( ?, ?, ?, ?, ? )",
        post_id,
        user_id,
        created,
        form.title,
        form.content,
    )
    .execute(&data.database)
    .await
    .expect("failed to insert post into database");

    resp.insert_header((
        header::LOCATION,
        HeaderValue::from_str(&format!("/post-{post_id}")).expect("invalid header value"),
    ));

    return resp.finish();
}
