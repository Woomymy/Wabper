use super::schema::pastes;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "pastes"]
/// Paste element with text, title, author, ...
pub struct Paste {
    pub id: String,
    pub body: String,
    pub title: String,
    pub author: String,
    pub deletionpw: String,
}

impl Paste {
    /// Remove deletionpw from Paste {} to prevent sending it
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
/// Informations about running Wabper server
pub struct ServerInfo {
    /// Version of wabper-server
    version: &'static str,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
