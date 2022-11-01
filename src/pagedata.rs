use chrono::Utc;
use serde::Serialize;

use rusqlite::Connection;

#[derive(Serialize)]
pub struct PageState {
    time: String,
    posts: Vec<Post>,
}

impl PageState {
    pub fn generate(conn: &Connection) -> Self {
        let posts = Self::get_posts(conn);

        Self {
            time: Utc::now().to_rfc3339(),
            posts: posts,
        }
    }

    fn get_posts(conn: &Connection) -> Vec<Post> {
        let mut query = conn.prepare("SELECT id, title, text, author, author_id FROM posts").expect("Failed to get posts.");
        let post_result = query.query_map([], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                text: row.get(2)?,
                author: row.get(3)?,
                author_id: row.get(4)?,
            })
        }).expect("Failed to get posts.");

        let mut posts = Vec::new();

        for post in post_result {
            posts.push(post.unwrap());
        }

        return posts;
    }
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct Post {
    id: i32,
    title: String,
    text: String,
    author: String,
    author_id: i32,
}
