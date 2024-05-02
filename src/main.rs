mod application;

use std::env;
use application::error::ApplicationError;
use log::info;
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
    let args: Vec<String> = env::args().collect();
    let command = args.get(1)
        .map(|command|Command::from(command.to_owned()))
        .unwrap_or(Command::GUI);
    info!("Detected mode: {}", command);
    return match command {
        application::command::Command::GUI => running_gui(),
        application::command::Command::IMPORT => import(),
        application::command::Command::EXPORT => export()
    };
}

