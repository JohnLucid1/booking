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

use std::os::unix::net::UnixDatagram;

use dotenv::dotenv;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// mod db;
mod files;
mod models;
use files::{Book, get_files};
use mongodb::{options::DatabaseOptions, Database};

// DONE: make a function that loops through a dir of books and saves all info about them
// DONE: ENV file with the directory of books 
// TODO: basic post that returns info about all books



#[get("/")] 
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let book_dir = std::env::var("BOOK_DIRECTORY").expect("No env variable");
    // let uri = std::env::var("MONGO_URI").expect("No mongodb env var");
    // let db_name = std::env::var("DB_CLIENT").expect("No DB_CLIENT in evn::var");
    // let db_collection = std::env::var("DB_COLLECTION").expect("No DB_COLLECTION in evn::var");

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


    // let all_files = get_files(&book_dir).expect("Couldn't parse directory");
    // let books: Vec<Book> = Book::process_directory(all_files);
    // let client = get_connection(&uri).await.unwrap();
    // let database = client.database(&db_name);
    // let collection = database.collection::<Book>(&db_collection);
    // db::create_new_books(books, &collection).await?;
}

