use std::{env, fs::read_to_string};
use application::{error::ApplicationError, reference::reference::{create, Reference}};
use crate::application::{command::match_command, database::{ensuring_model, opening_database}, gui::gui::TemplateApp};


mod application;

fn main() -> Result<(), ApplicationError> {
    // MATCH COMMANDS AND DO WHATS NEEDED
    let args: Vec<String> = env::args().collect();
    let command = match_command(args.get(1).unwrap_or(&"gui".to_string()));

    let init = opening_database();
    ensuring_model();
    match init {
        Ok(_) => println!("Database initialized"),
        Err(err) => panic!("Error: {}", err)
    }

    match command {
        application::command::Command::GUI => running_gui(),
        application::command::Command::IMPORT => import()
    }
}

fn running_gui() -> Result<(), ApplicationError>{
    

    // OPEN GUI
    println!("Getting gui context");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    ).map_err(ApplicationError::from)
}



fn import() -> Result<(), ApplicationError> {
    return read_to_string("import.csv") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from).try_for_each(|line| create(&Reference::from(line)));
}

type CsvLine = String;