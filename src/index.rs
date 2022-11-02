use actix_web::{
    get,
    http::header::{self, ContentType, HeaderValue},
    web::Data,
    HttpRequest, HttpResponse,
};
use serde::Serialize;

use crate::{post::Post, AppState};

pub const INDEX_TEMPLATE: &'static str = include_str!("templates/index.hbs");

#[derive(Serialize)]
pub struct IndexPageState {
    posts: Vec<Post>,
    user_id: u32,
}

#[get("/")]
pub async fn get_index(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    let (new_user_token, user_id) = crate::manage_cookies(&req, &data).await;

    let posts: Vec<Post> = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&data.database)
        .await
        .unwrap();

    let pagestate = IndexPageState { posts, user_id };
    let page = data.template_registry.render("index", &pagestate).unwrap();

    let mut resp = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(page);

    if let Some(user_token) = new_user_token {
        resp.headers_mut().insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!("userToken={user_token}"))
                .expect("invalud header value"),
        );
    }

    return resp;
}
