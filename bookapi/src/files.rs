use epub::doc::EpubDoc;
use std::{
    fs::{read_dir, File},
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use crate::models::Book;

impl Book {
    pub fn new_book(path: PathBuf) -> Book {
        let doc = EpubDoc::new(&path).expect("Couldn't parse epub");
        Book {
            title: get_epub_key(&doc, "title"),
            author: get_epub_key(&doc, "creator"),
            year: get_epub_key(&doc, "date"),
            publisher: get_epub_key(&doc, "publisher"),
            genres: doc
                .metadata
                .get("subject")
                .unwrap_or(&vec!["none".to_string()])
                .to_owned(),
            path: path.to_string_lossy().to_string(),
        }
    }

    pub fn process_directory(dirs: Vec<PathBuf>) -> Vec<Book> {
        let file_info_list: Arc<Mutex<Vec<Book>>> = Arc::new(Mutex::new(Vec::new()));
        let handles: Vec<_> = dirs
            .into_iter()
            .map(|file_path| {
                let file_info_list = Arc::clone(&file_info_list);
                thread::spawn(move || {
                    let doc = EpubDoc::new(&file_path).expect("Couldn't parse epub");
                    let new_book = Book {
                        title: get_epub_key(&doc, "title"),
                        author: get_epub_key(&doc, "creator"),
                        year: get_epub_key(&doc, "date"),
                        publisher: get_epub_key(&doc, "publisher"),
                        genres: doc
                            .metadata
                            .get("subject")
                            .unwrap_or(&vec!["none".to_string()])
                            .to_owned(),
                        path: file_path.to_string_lossy().to_string(),
                    };

                    let mut info_list = file_info_list.try_lock().unwrap();
                    info_list.push(new_book);
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Failed to join thread");
        }

        return Arc::try_unwrap(file_info_list)
            .unwrap()
            .into_inner()
            .unwrap();
    }
}

fn get_epub_key(doc: &EpubDoc<BufReader<File>>, sub: &str) -> String {
    return doc
        .metadata
        .get(sub)
        .unwrap_or(&vec!["None".to_string()])
        .first()
        .unwrap()
        .to_owned();
}

fn get_files(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let res = read_dir(dir)?;
    let entryes: Vec<PathBuf> = res.filter_map(|e| e.ok().map(|f| f.path())).collect();
    Ok(entryes)
}
