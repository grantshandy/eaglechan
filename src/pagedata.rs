use serde::Serialize;

use sqlx::{sqlite, SqlitePool};

#[derive(Serialize)]
pub struct PageState {
    posts: Vec<Post>,
}

impl PageState {
    pub async fn generate(pool: &SqlitePool) -> PageState {
        let posts: Vec<Post> = sqlx::query_as!(Post, "SELECT id, title, content, author FROM posts")
            .fetch_all(pool)
            .await
            .unwrap();

        return PageState { posts };
    }
}

#[derive(Serialize, Debug)]
pub struct Post {
    id: i64,
    title: String,
    content: String,
    author: String,
}
