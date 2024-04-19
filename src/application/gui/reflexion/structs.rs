use std::{fs::{read_to_string, File}, io::Write};

use egui::{TextEdit, Ui, Window};
use log::info;

use crate::application::{error::ApplicationError, reflexion::{service::get_all, structs::Reflexion}};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReflexion {
    pub reflexion: Reflexion,
    pub list_reflexions: Vec<Reflexion>,
    pub edit: EditText,
    pub edit_reflexion: EditReflexion
}

impl SectionReflexion {
    pub fn new() -> Self {
        Self {
            reflexion: Reflexion::new(),
            list_reflexions: get_all().unwrap_or_default(),
            edit: EditText::default(),
            edit_reflexion: EditReflexion {show: false, reflexion: Reflexion::new(), contenu: String::from("")}
            
        }
    } 
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct EditText {}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct EditReflexion {
    pub show: bool,
    pub reflexion: Reflexion,
    pub contenu: String
}


impl EditText {
    pub fn show(&mut self, ui: &mut Ui,   edit_reflexion: &mut EditReflexion) -> Result<(), ApplicationError> {

        let path = edit_reflexion.reflexion.get_path();
        Window::new(&edit_reflexion.reflexion.sujet)
            .open(&mut edit_reflexion.show)
            .resizable(true)
            .default_size([300.0, 300.0])
            .max_height(300.0)
            .show(ui.ctx(), 
            |ui|
            {
                ui.add_sized(ui.available_size(), TextEdit::multiline(&mut edit_reflexion.contenu));
            
                if ui.button("Enregistrer").clicked() {
                    let write = File::options().read(true).write(true).open(&path)
                        .and_then(|mut f| 
                            f.write_all(edit_reflexion.contenu.as_bytes()));
                    match write {
                        Err(e) => info!("Error while writing file {} :  {}", path, e.to_string()),
                        Ok(_) => info!("")
                    }
                } 
            });
            Ok(())
    }

    pub fn open(&mut self ,reflexion: Reflexion, edit_reflexion:&mut EditReflexion) {
        info!("Opening: {}", reflexion.get_path());
        edit_reflexion.contenu = read_to_string( reflexion.get_path()).unwrap();
        info!("Contenu: {}", edit_reflexion.contenu);
        edit_reflexion.show = !edit_reflexion.show;
        edit_reflexion.reflexion = reflexion;
    }

}