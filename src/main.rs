mod reference;
mod notes;
mod graph;
mod finance;
mod database;
mod command;
mod application_error;
mod file;
mod gui;
mod server;
mod connecteur;
mod client;

use std::env;
use command::Command;
use application_error::ApplicationError;
use connecteur::Connecteur;
use file::{ensuring_storage, export, import, ModeExport};
use gui::app::running_gui;
use log::{info, warn};
use dotenv::dotenv;
use database::{ensuring_model, opening_database};

fn main() -> Result<(), ApplicationError> {
    dotenv().ok();
    env_logger::init();
    info!("Application Initialization");
    


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
    let mode_connecteur = std::env::var("MODE")
        .map(|v|Connecteur::from_str(&v))
        .unwrap_or_else(|e| {
            warn!("Erreurs lors de la lecture du mode, mise en mode LOCAL par defaut: {}", e);
            Connecteur::LOCAL
        });

    if mode_connecteur == Connecteur::LOCAL {
        opening_database()
        .and_then(|_| ensuring_model())
        .and_then(|_| ensuring_storage())?;
    }
    
    let mode_export = std::env::var("EXPORT")
    .map(ModeExport::try_from)
    .unwrap_or_else(|e| {
        warn!("Erreurs lors de la lecture du mode d'export, mise en mode CSV par defaut: {}", e);
        Ok(ModeExport::CSV)
    })?;

    match command {
        command::Command::Gui => running_gui(mode_connecteur),
        command::Command::Import => import(),
        command::Command::Export => export(mode_export),
        command::Command::Server => server::server::server(),
    }
}