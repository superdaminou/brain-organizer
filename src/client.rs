use reqwest::{blocking::{Body, Response}, header::HeaderMap};


pub fn get(path: &String) -> Response {
    let client = reqwest::blocking::Client::new();
    let mut headers= HeaderMap::new();
    headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
    let user = std::env::var("USER").expect("Missing url");
    let password = std::env::var("PASSWORD").expect("Missing url");
    
    let url = std::env::var("SERVER_URL").expect("Missing url");
    client.get(format!("{}{}", url, path))
        .headers(headers.clone())
        .basic_auth(user, Some(password))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        
}

pub fn post(path: &String, body: Body) -> Response {
    let client = reqwest::blocking::Client::new();
    let mut headers= HeaderMap::new();
    headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
    let user = std::env::var("USER").expect("Missing url");
    let password = std::env::var("PASSWORD").expect("Missing url");
    //let auth = BASE64_STANDARD.encode(format!("{}:{}", user, password));
    //headers.insert("Authorization", format!("Basic {}", auth).parse().unwrap());
    
    let url = std::env::var("SERVER_URL").expect("Missing url");
    client.post(format!("{}{}", url, path))
        .headers(headers.clone())
        .basic_auth(user, Some(password))
        .body(body)
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
}


pub fn delete(path: &String) -> Response {
    let client = reqwest::blocking::Client::new();
    let mut headers= HeaderMap::new();
    headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
    let user = std::env::var("USER").expect("Missing url");
    let password = std::env::var("PASSWORD").expect("Missing url");
    
    let url = std::env::var("SERVER_URL").expect("Missing url");
    client.delete(format!("{}{}", url, path))
        .headers(headers.clone())
        .basic_auth(user, Some(password))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
}

pub fn update(path: &String, body: Body) -> Response {
    let client = reqwest::blocking::Client::new();
    let mut headers= HeaderMap::new();
    headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
    let user = std::env::var("USER").expect("Missing url");
    let password = std::env::var("PASSWORD").expect("Missing url");
    
    let url = std::env::var("SERVER_URL").expect("Missing url");
    client.put(format!("{}{}", url, path))
        .headers(headers.clone())
        .basic_auth(user, Some(password))
        .body(body)
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
}