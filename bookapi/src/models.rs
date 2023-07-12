use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub pagecount: u16,
    pub publisher: String,
    pub genres: Vec<String>,
    pub path: String,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "some title".to_string(),
            author: "some author".to_string(),
            year: 1900,
            pagecount: 300,
            publisher: "New york".to_string(),
            genres: vec!["cringe".to_string(), "cringe".to_string()],
            path: "./books/cringebook.epub".to_string(),
        }
    }
}
