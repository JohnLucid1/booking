// use std::fs::File;
// use std::io::Read;
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

fn main(){ 
    let message: String = String::from("hello world");
    
    
    let clojure = move || {
        println!("{}", message);
    };

    clojure();
}
