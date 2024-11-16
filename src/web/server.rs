use ilmen_http::{http::security::service::SecurityProtocol, HttpServer, Route, Verb};
use crate::application_error::ApplicationError;
use super::references_controller;


pub fn web() -> Result<(), ApplicationError> {
    let configuration = ilmen_http::Config::new(8080, SecurityProtocol::Basic(base_auth));
    let server = HttpServer::new(configuration, routes());
    server.start();
    Ok(())
}



pub fn routes() -> Vec<Route> {
    let routes = vec![
        Route::new(&Verb::GET, "/references" ,references_controller::get_all, true),
        Route::new(&Verb::GET, "/references/{id}", references_controller::get_one, true),
        Route::new(&Verb::POST, "/references" ,  references_controller::post_one, true),
        Route::new(&Verb::POST, "/references/search", references_controller::search, true),
        ];

    routes
}

fn base_auth(couple: (String, String)) -> bool {
    couple.0 == "admin" && couple.1 == std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set for baseAuth")
}