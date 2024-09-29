use ilmen_http::{http::security::service::SecurityProtocol, HttpServer, Route, Verb};

use crate::error::ApplicationError;

use super::references_controller::{get_all, get_one};


pub fn web() -> Result<(), ApplicationError> {
    let configuration = ilmen_http::Config::new(8080, SecurityProtocol::Basic(base_auth));
    let server = HttpServer::new(configuration, routes());
    server.start();
    Ok(())
}



pub fn routes() -> Vec<Route> {
    let routes = vec![
        Route {verb: Verb::GET, route: "/references".to_string(),method: get_all, need_security: true},
        Route {verb: Verb::GET, route: "/references/{id}".to_string(),method: get_one, need_security: true},
        ];

    routes
}

fn base_auth(couple: (String, String)) -> bool {
    couple.0 == "admin" && couple.1 == std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set for baseAuth")
}