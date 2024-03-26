use std::{fs::File, io::{Error, Read}};

pub fn create_file(path: &str) -> Result<File, std::io::Error> {
    println!("Creating database file: {}", path);
    return File::create(path);
}

pub fn opening_file(path: &str) -> Result<File, std::io::Error> {
    println!("Try opening database file: {}", path);
    return File::open(path);
}

fn  get_all_from_file(path: &str) -> Result<usize, Error> {
    let mut content = String::new();
    return opening_file(path)?.read_to_string(&mut content);
}