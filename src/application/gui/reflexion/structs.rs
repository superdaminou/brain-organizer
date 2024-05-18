use std::{fs::{read_to_string, File}, io::Write};

use egui::{TextEdit, Ui, Window};
use log::info;

use crate::application::{error::ApplicationError, file::construct_path, gui::structs::Fenetre, reflexion::{service::ReflexionDatabase, Reflexion}};

use super::gui::section_reflexions;
use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReflexion {
    pub reflexion: Reflexion,
    pub list_reflexions: Vec<Reflexion>,
    pub edit: EditText,
    pub edit_reflexion: EditReflexion
}


impl Default for SectionReflexion {
    fn default() -> Self {
        Self {
            reflexion: Reflexion::new(),
            list_reflexions: Reflexion::get_all().unwrap_or_default(),
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
    pub fn show(&mut self, ui: &mut Ui,   edit_reflexion: &mut EditReflexion) -> Result<()> {
        Window::new(&edit_reflexion.reflexion.sujet)
            .open(&mut edit_reflexion.show)
            .vscroll(true)
            .default_size([300.0, 300.0])
            .show(ui.ctx(), 
            |ui|
            {
                ui.add_sized([ui.available_height() -50.0, ui.available_width() - 50.0], TextEdit::multiline(&mut edit_reflexion.contenu));
            
                if ui.button("Enregistrer").clicked() {
                    let write = File::options().read(true).write(true).open(construct_path(&edit_reflexion.reflexion))
                        .and_then(|mut f| 
                            f.write_all(edit_reflexion.contenu.as_bytes()));
                    match write {
                        Err(e) => info!("Error while writing file {} :  {}", construct_path(&edit_reflexion.reflexion), e.to_string()),
                        Ok(_) => info!("")
                    }
                } 
            });
            Ok(())
    }

    pub fn open(&mut self ,reflexion: Reflexion, edit_reflexion:&mut EditReflexion) {
        info!("Opening: {}", construct_path(&reflexion));
        edit_reflexion.contenu = read_to_string( construct_path(&reflexion)).unwrap();
        edit_reflexion.show = !edit_reflexion.show;
        edit_reflexion.reflexion = reflexion;
    }

}


impl Fenetre for SectionReflexion {
    fn name(&self) -> &'static str {
        "Reflexions"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        egui::Window::new(self.name())
        .open(is_open)
        .scroll2(true)
        .show(ctx, |ui| {
            section_reflexions(self, ui)
        });
        Ok(())
    }
}