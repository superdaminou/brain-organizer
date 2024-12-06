use std::{fs::{read_to_string, File}, io::Write};

use egui::{TextEdit, Ui, Window};
use anyhow::Result;

use crate::{application_error::ApplicationError, file::construct_path, notes::{self, Note}};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct EditText {}

#[derive(serde::Deserialize, serde::Serialize,Default, Debug, Clone)]
pub struct  EditFileable <T: Fileable>{
    pub show: bool,
    pub note: T
}

impl EditText {
    pub fn show<T: Fileable>(&mut self, ui: &mut Ui,   edit_file: &mut EditFileable<T>) -> Result<Option<String>> {
        Window::new(&edit_file.note.filename())
            .open(&mut edit_file.show)
            .vscroll(true)
            .default_size([300.0, 300.0])
            .show(ui.ctx(), 
            |ui|
            {

                ui.add_sized([
                    ui.available_height() -50.0, ui.available_width() - 50.0], 
                    TextEdit::multiline(&mut edit_file.note.contenu())
                        .font(egui::TextStyle::Monospace)
                        .code_editor());
            
                if ui.button("Enregistrer").clicked() {
                    T::write(&edit_file.note);
                } 

            });
            Ok(None)
    }

    pub fn open<T: Fileable + Clone>(&mut self ,note: &T, edit_reflexion:&mut EditFileable<T>) -> Result<(), ApplicationError> {
        edit_reflexion.note = note.clone();
        edit_reflexion.show = !edit_reflexion.show;
        Ok(())
    }
}


pub trait Fileable {
    fn id(&self) -> String;
    fn filename(&self) -> String;
    fn contenu(&self) -> String;
    fn write<T: Fileable>(file: &T) -> Result<()>;
}

