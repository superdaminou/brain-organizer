use ilmen_http::{http::security::service::SecurityProtocol, HttpServer, Route, Verb};
use crate::application_error::ApplicationError;
use super::{note_controller, references_controller, tags_controller};


pub fn server() -> Result<(), ApplicationError> {
    let port = std::env::var("PORT").expect("PORT must be defined").parse::<i32>().expect("invalid PORT");
    let configuration = ilmen_http::Config::initialize()
        .with_adresse("0.0.0.0", &port)
        .with_security(&SecurityProtocol::Basic(base_auth))
        .to_owned();
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
        Route::new(&Verb::PUT, "/references/{id}", references_controller::update_one, true),
        Route::new(&Verb::DELETE, "/references/{id}", references_controller::delete, true),
        
        Route::new(&Verb::GET, "/tags" ,tags_controller::all_tags_distinct, true),

        Route::new(&Verb::GET, "/notes" ,note_controller::get_all, true),
        Route::new(&Verb::GET, "/notes/{id}", note_controller::get_one, true),
        Route::new(&Verb::POST, "/notes" ,  note_controller::post_one, true),
        Route::new(&Verb::PUT, "/notes/{id}", note_controller::update_one, true),
        Route::new(&Verb::DELETE, "/notes/{id}", note_controller::delete, true),
        ];

    routes
}

fn base_auth(couple: (String, String)) -> bool {
    couple.0 == std::env::var("USER").expect("USER must be set for baseAuth") && couple.1 == std::env::var("PASSWORD").expect("PASSWORD must be set for baseAuth")
}