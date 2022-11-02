use actix_web::{get, web::Data, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::{AppState, Post};

pub const TEMPLATE: &'static str = include_str!("index.hbs");

#[derive(Serialize)]
struct PageState {
    posts: Vec<Post>,
    user_id: String,
}

#[get("/")]
pub async fn get_index(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    let mut resp = HttpResponse::Ok();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let posts: Vec<Post> = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&data.database)
        .await
        .unwrap()
        .into_iter()
        .rev()
        .collect();

    let page: String = data
        .template_registry
        .render(
            "index",
            &PageState {
                user_id,
                posts,
            },
        )
        .unwrap();

    return resp.body(page);
}
