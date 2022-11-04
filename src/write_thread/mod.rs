use actix_web::{get, web::Data, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::AppState;

pub const TEMPLATE: &'static str = include_str!("write_thread.hbs");

#[derive(Serialize)]
struct PageState {
    user_id: String,
}

#[get("/write")]
pub async fn get_write_thread(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    let mut resp = HttpResponse::Ok();
    let user_id = crate::manage_cookies(&req, &data, &mut resp).await;

    let page = data
        .template_registry
        .render("write_thread", &PageState { user_id })
        .expect("couldn't render write page");

    resp.body(page)
}
