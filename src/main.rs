mod application;

use std::env;
use application::{error::ApplicationError, file::lib::{ensuring_storage, export, import}};
use log::info;
use crate::application::{command::{match_command, Command}, database::{ensuring_model, opening_database}, gui::app::running_gui};


fn main() -> Result<(), ApplicationError> {
    info!("Application Initialization");
    opening_database()
    .and_then(|_| ensuring_model())
    .and_then(|_| ensuring_storage())?;


    // MATCH COMMANDS AND DO WHATS NEEDED
    let args: Vec<String> = env::args().collect();
    let command = match_command(args.get(1).unwrap_or(&Command::GUI.to_string()));
    info!("Detected mode: {}", command);
    match command {
        application::command::Command::GUI => running_gui(),
        application::command::Command::IMPORT => import(),
        application::command::Command::EXPORT => export()
    }
}

