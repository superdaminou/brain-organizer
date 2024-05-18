mod application;

use std::env;
use application::error::ApplicationError;
use log::{info, error};
use crate::application::{command::Command, database::{ensuring_model, opening_database}, file::{ensuring_storage, export, import}, gui::app::running_gui};
use dotenv::dotenv;

fn main() -> Result<(), ApplicationError> {
    dotenv().ok();
    env_logger::init();
    info!("Application Initialization");
    opening_database()
    .and_then(|_| ensuring_model())
    .and_then(|_| ensuring_storage())?;


    // MATCH COMMANDS AND DO WHATS NEEDED
    let command = match Command::try_from(env::args().skip(1).collect::<Vec<String>>()) {
        Ok(command) => {
            info!("Detected mode: {}", command);
            command
        },
        Err(error) => {
            error!("{}", error);
            Command::Gui
        } 
    };
    
    match command {
        application::command::Command::Gui => running_gui(),
        application::command::Command::Import => import(),
        application::command::Command::Export => export()
    }
}

