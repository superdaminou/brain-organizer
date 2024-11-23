mod reference;
mod reflexion;
mod graph;
mod finance;
mod database;
mod command;
mod application_error;
mod file;
mod gui;
mod server;
mod connecteur;

use std::env;
use command::Command;
use application_error::ApplicationError;
use file::{ensuring_storage, export, import};
use gui::app::running_gui;
use log::{info, warn};
use dotenv::dotenv;
use database::{ensuring_model, opening_database};

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
            warn!("Could not determine mode:  {}", error);
            Command::Gui
        } 
    };
    
    match command {
        command::Command::Gui => running_gui(),
        command::Command::Import => import(),
        command::Command::Export => export(),
        command::Command::Web => server::server::web(),
    }
}