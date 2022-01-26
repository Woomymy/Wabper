use super::schema::pastes;
#[derive(Insertable, Clone, Serialize, Deserialize)]
#[table_name = "pastes"]
pub struct NewPaste {
    pub body: String,
    pub title: String,
    pub author: String,
}
