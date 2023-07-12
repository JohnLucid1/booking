use std::{error::Error, io::ErrorKind};

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    results::DeleteResult,
    Client, Collection,
};
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

pub async fn create_new_book(
    book: Book,
    collection: &Collection<Book>,
) -> Result<(), mongodb::error::Error> {
    collection.insert_one(book, None).await?;

    Ok(())
}

pub async fn delete_book(
    book_id: &str,
    collection: &Collection<Book>,
) -> Result<DeleteResult, mongodb::error::Error> {
    let filter = doc! {"id": book_id};

    match collection.delete_one(filter, None).await {
        Ok(message) => Ok(message),
        Err(err) => Err(err),
    }
    // Ok(String::from("Successfully deleted book"))
}

pub async fn find_book(
    book_id: &str,
    collection: &Collection<Book>,
) -> Result<Book, mongodb::error::Error> {
    let filter = doc! {"id": book_id};

    let result = collection.find_one(filter, None).await;

    match result {
        Ok(book) => match book {
            Some(book) => Ok(book),
            None => Err(mongodb::error::Error::custom("not found")),
        },
        Err(err) => Err(err),
    }
}

pub async fn get_connection(uri: &str) -> Result<Client, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    Ok(client)
}
