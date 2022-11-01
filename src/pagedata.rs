use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct PageState {
    time: String,
    posts: Vec<Post>,
}

impl PageState {
    pub fn generate() -> Self {
        Self {
            time: Utc::now().to_rfc3339(),
            posts: vec![
                Post {
                    id: "123456769384745".to_string(),
                    title: "why foo is better than bar".to_string(),
                    text: "this is why... blah blah blah blah....".to_string(),
                    author: "qlawiueyrt03q45jhgiuyqwert".to_string(),
                },
                Post {
                    id: "1234857957892834759".to_string(),
                    title: "why bar is supreme".to_string(),
                    text: "this is why... asdfasdfasdfasdf".to_string(),
                    author: "qlawiueyrt03q45jhgiuyqwert".to_string(),
                },
                Post {
                    id: "11349587123419234875985".to_string(),
                    title: "why zig is secretly the best".to_string(),
                    text: "loris whatever...".to_string(),
                    author: "qlawiueyrt03q45jhgiuyqwert".to_string(),
                },
            ],
        }
    }
}

#[derive(Serialize)]
pub struct Post {
    id: String,
    title: String,
    text: String,
    author: String,
}
