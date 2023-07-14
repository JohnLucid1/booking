use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: String,
    pub publisher: String,
    pub genres: Vec<String>,
    pub path: String,
}




impl Default for Book {
    fn default() -> Self {
        Self {
            title: "Dune".to_string(),
            author: "Frank Herbert".to_string(),
            year: "1231.1321".to_string(),
            publisher: "Chilton Books".to_string(),
            genres: vec!["SCI FI".to_string()],
            path: "./books/Dune - Frank Herbert.epub".to_string(),
        }
    }

}