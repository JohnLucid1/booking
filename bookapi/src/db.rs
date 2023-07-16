use std::{error::Error, io::ErrorKind};

use async_std::{io::Cursor, stream::StreamExt};
use mongodb::{
    bson::{doc, from_document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    results::DeleteResult,
    Client, Collection,
};
use serde::{Deserialize, Serialize};

use crate::models::Book;

pub async fn create_new_book(
    book: Book,
    collection: &Collection<Book>,
) -> Result<(), mongodb::error::Error> {
    collection.insert_one(book, None).await?;

    Ok(())
}

pub async fn create_new_books(
    books: Vec<Book>,
    collection: &Collection<Book>,
) -> Result<(), mongodb::error::Error> {
    collection.insert_many(books, None).await?;
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
}

pub async fn get_all_books(
    collection: &Collection<Book>,
) -> Result<Vec<Book>, mongodb::error::Error> {
    let cursor = collection.find(None, None).await?;

    let mut book_vec: Vec<Book> = Vec::new();
    // for result in cursor.  {
    //     if let Ok(document) = result {
    //         let book:Book = from_document(document)?;
    //         book_vec.push(book);
    //     }
    // }

    // cursor.map(|result| {
    //     if let Ok(document) = result {
    //         let book:Book = from_document(document)?;
    //         book_vec.push(book)
    //     }
    // });

    cursor.map(|res| match res {
        Ok(some) => book_vec.push(some),
        Err(err) => eprintln!("ERROR: {err}"),
    });

    Ok(book_vec)
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
