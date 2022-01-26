use super::schema::pastes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Insertable, Clone, Serialize, Deserialize)]
#[table_name = "pastes"]
pub struct NewPaste {
    pub body: String,
    pub title: String,
    pub author: String,
}
