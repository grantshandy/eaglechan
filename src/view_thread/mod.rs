use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, HttpResponse,
};
use serde::Serialize;

use crate::{AppState, DATE_FORMATTING};

pub const TEMPLATE: &'static str = include_str!("view_thread.hbs");

#[derive(Serialize)]
struct Thread {
    thread_id: String,
    user_id: String,
    created: String,
    last_updated: String,
    title: String,
    content: String,
}

#[derive(Serialize)]
struct Comment {
    thread_id: String,
    user_id: String,
    created: String,
    content: String,
}

#[derive(Serialize)]
struct PageState {
    num_comments: usize,
    comments: Vec<Comment>,
    thread: Thread,
    user_id: String,
}

#[get("/thread/{thread_id}")]
pub async fn get_thread(
    req: HttpRequest,
    data: Data<AppState>,
    thread_id: Path<String>,
) -> HttpResponse {
    let mut resp = HttpResponse::Ok();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let thread_id = thread_id.to_string();

    let thread_record = sqlx::query!(
        "SELECT * FROM threads WHERE thread_id = ? LIMIT 1",
        thread_id
    )
    .fetch_one(&data.database)
    .await
    .expect("no post found");

    let thread = Thread {
        thread_id: thread_record.thread_id,
        user_id: thread_record.user_id,
        created: thread_record
            .created
            .format(DATE_FORMATTING)
            .to_string(),
        last_updated: thread_record
            .last_updated
            .format(DATE_FORMATTING)
            .to_string(),
        title: thread_record.title,
        content: thread_record.content,
    };

    let comments: Vec<Comment> = sqlx::query!(
        "SELECT * FROM comments WHERE thread_id = ? ORDER BY created ASC",
        thread_id
    )
    .fetch_all(&data.database)
    .await
    .expect("failed to get comments")
    .iter()
    .map(|record| Comment {
        content: record.content.clone(),
        created: record.created.format(DATE_FORMATTING).to_string(),
        thread_id: record.thread_id.clone(),
        user_id: record.user_id.clone(),
    })
    .collect();

    let page = data
        .template_registry
        .render(
            "view_thread",
            &PageState {
                user_id,
                num_comments: comments.len(),
                comments,
                thread,
            },
        )
        .unwrap();

    return resp.body(page);
}
