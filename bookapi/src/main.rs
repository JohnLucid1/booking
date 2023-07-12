// use std::fs::File; // use std::io::Read;
// use tide::Request;
// use tide::Response;
// async fn send_file(req: Request<()>) -> tide::Result<Response> {
//     let mut file = File::open("./somefile.txt")?;

//     let mut buffer: Vec<u8> = Vec::new();
//     file.read_to_end(&mut buffer)?;

//     let mut response = Response::new(200);
//     response.set_body(buffer);
//     response.insert_header("Content-Type", "applications/octet-stream");
//     response.insert_header("Content-Disposition", "attachment; filename=somefile.txt;");

//     Ok(response)
// }

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     let mut app = tide::new();
//     app.at("/file").get(send_file);
//     app.listen("127.0.0.1:8080").await?;

//     Ok(())
// }

mod db;
use crate::db::{create_new_book, delete_book, find_book, get_connection, Book};

#[async_std::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "mongodb://127.0.0.1:27017/?maxPoolSize=20&w=majority";

    let client = get_connection(uri).await.unwrap();
    let database = client.database("ivan");
    let collection = database.collection::<Book>("books");
    // let book = Book::default();

    // match create_new_book(book, &collection).await {
    //     Ok(()) => println!("New book created"),
    //     Err(err) => eprintln!("ERROR: {}", err),
    // }

    let message = delete_book(
        "fb121f45-c30c-401d-a386-ef396bf0f649",
        &collection,
    )
    .await;
    println!("{:#?}", message);

    Ok(())
}
