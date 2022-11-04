use actix_web::{get, web::Data, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::{AppState, DATE_FORMATTING};

pub const TEMPLATE: &'static str = include_str!("index.hbs");

const TITLE_CHAR_LIMIT: usize = 60;
const CONTENT_CHAR_LIMIT: usize = 700;

#[derive(Serialize)]
struct Thread {
    thread_id: String,
    user_id: String,
    created: String,
    last_updated: String,
    title: String,
    content: String,
    overflow: bool,
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
            let mut title = x.title;

            if title.len() > TITLE_CHAR_LIMIT {
                title = truncate_by_chars(title, TITLE_CHAR_LIMIT);
                title.push_str("...");
            }

            let mut content = x.content;

            let overflow = if content.len() > CONTENT_CHAR_LIMIT {
                println!("{}", content.len());
                content = truncate_by_chars(content, CONTENT_CHAR_LIMIT);
                content.push_str("...");

                true
            } else {
                false
            };

            Thread {
                thread_id: x.thread_id,
                user_id: x.user_id,
                created: x.created.format(DATE_FORMATTING).to_string(),
                last_updated: x.last_updated.format(DATE_FORMATTING).to_string(),
                title,
                content,
                overflow,
            }
        })
        .collect();

    let num_threads = threads.len();

    let page: String = data
        .template_registry
        .render(
            "index",
            &PageState {
                threads,
                num_threads,
                user_id,
            },
        )
        .unwrap();

    return resp.body(page);
}

fn truncate_by_chars(s: String, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}
