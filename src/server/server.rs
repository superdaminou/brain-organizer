use ilmen_http::{http::security::service::SecurityProtocol, HttpServer, Route, Verb};
use crate::application_error::ApplicationError;
use super::{references_controller, tags_controller};


pub fn web() -> Result<(), ApplicationError> {
    let port = std::env::var("PORT").expect("PORT must be set for baseAuth").parse::<i32>().expect("PORT non valide");
    let configuration = ilmen_http::Config::new( port, SecurityProtocol::Basic(base_auth));
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
        ];

    routes
}

fn base_auth(couple: (String, String)) -> bool {
    couple.0 == std::env::var("USER").expect("USER must be set for baseAuth") && couple.1 == std::env::var("PASSWORD").expect("PASSWORD must be set for baseAuth")
}