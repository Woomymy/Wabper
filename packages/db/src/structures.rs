use super::schema::pastes;
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "pastes"]
pub struct Paste {
    pub id: String,
    pub body: String,
    pub title: String,
    pub author: String,
    pub deletionpw: String,
}

impl Paste {
    pub fn without_delete_pw(&self) -> Paste {
        Paste {
            id: self.id.clone(),
            body: self.body.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            deletionpw: "HIDDEN".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    version: &'static str,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
