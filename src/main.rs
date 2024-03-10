use std::{env, fmt::Error, fs::File};
use application::command::Command;
use crate::application::app::TemplateApp;
use crate::application::command::match_command;
use crate::application::database::{ensuring_model, opening_database};


mod application;

fn main() -> eframe::Result<()> {
    println!("Hello, world!");
    //dbg!(args); => Debug LINE 
    

    // MATCH COMMANDS AND DO WHATS NEEDED
    let args: Vec<String> = env::args().collect();
    /*
    let command = match args.iter().next() {
        Some(value) => match_command(value),
        None => panic!("No command provided")
    }; 
    */


    // OPENING DATABASE AND GETTING CONNECTION
    let database = opening_database();
     match database {
        Ok(file) => ensuring_model(file),
        Err(error) => panic!("An error occured: {}", error.to_string())
    };


    print!("Getting gui context");

    // OPEN GUI
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
    )
}




fn apply_operation(file: File, command: Command) -> Result<String, Error> {
    return Ok("String".to_string());

}