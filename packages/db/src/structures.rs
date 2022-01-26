use super::schema::pastes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "pastes"]
pub struct Paste {
    pub id: String,
    pub body: String,
    pub title: String,
    pub author: String,
}
