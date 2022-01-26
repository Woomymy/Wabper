use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Paste {
    pub id: i32,
    pub body: String,
    pub title: String,
    pub author: String,
}
