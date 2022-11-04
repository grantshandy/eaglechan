use actix_web::{get, web::Data, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::{AppState, DATE_FORMATTING};

pub const TEMPLATE: &'static str = include_str!("index.hbs");

#[derive(Serialize)]
struct Thread {
    thread_id: String,
    user_id: String,
    created: String,
    last_updated: String,
    title: String,
    content: String,
    // num_comments: usize,
}

#[derive(Serialize)]
struct PageState {
    num_threads: usize,
    threads: Vec<Thread>,
    user_id: String,
}

#[get("/")]
pub async fn get_index(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    let mut resp = HttpResponse::Ok();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let threads: Vec<Thread> = sqlx::query!("SELECT * FROM threads ORDER BY last_updated DESC")
        .fetch_all(&data.database)
        .await
        .unwrap()
        .into_iter()
        .map(|x| {
            Thread {
                thread_id: x.thread_id,
                user_id: x.user_id,
                created: x.created.format(DATE_FORMATTING).to_string(),
                last_updated: x.last_updated.format(DATE_FORMATTING).to_string(),
                title: x.title,
                content: x.content,
            }
        })
        .collect();
    
        let num_threads = threads.len();

    let page: String = data
        .template_registry
        .render("index", &PageState { threads, num_threads, user_id })
        .unwrap();

    return resp.body(page);
}
