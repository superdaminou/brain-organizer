use std::{fs::{read_to_string, File}, io::Write};

use egui::{TextEdit, Ui, Window};
use anyhow::Result;
use log::info;

use crate::{error::ApplicationError, file::construct_path};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct EditText {}

#[derive(serde::Deserialize, serde::Serialize,Default, Debug, Clone)]
pub struct EditFile {
    pub show: bool,
    pub filename: String,
    pub contenu: String
}

impl EditText {
    pub fn show(&mut self, ui: &mut Ui,   edit_file: &mut EditFile) -> Result<()> {
        Window::new(&edit_file.filename)
            .open(&mut edit_file.show)
            .vscroll(true)
            .default_size([300.0, 300.0])
            .show(ui.ctx(), 
            |ui|
            {

                ui.add_sized([
                    ui.available_height() -50.0, ui.available_width() - 50.0], 
                    TextEdit::multiline(&mut edit_file.contenu)
                        .font(egui::TextStyle::Monospace)
                        .code_editor());
            
                if ui.button("Enregistrer").clicked() {
                    let write = File::options()
                        .read(true)
                        .write(true)
                        .open(construct_path(&(edit_file.filename)))
                        .and_then(|mut f| 
                            f.write_all(edit_file.contenu.as_bytes()));
                    match write {
                        Err(e) => info!("Error while writing file {} :  {}", construct_path(&edit_file.filename), e.to_string()),
                        Ok(_) => info!("Saved: {}", &edit_file.filename)
                    }
                } 
            });
            Ok(())
    }

    pub fn open(&mut self ,filename: String, edit_reflexion:&mut EditFile) -> Result<(), ApplicationError> {
        info!("Opening: {}", construct_path(&filename));
        edit_reflexion.contenu = read_to_string(construct_path(&filename))
            .map_err(|e| ApplicationError::DefaultError("Could not open find".to_string()))?;
        edit_reflexion.show = !edit_reflexion.show;
        edit_reflexion.filename = filename;
        Ok(())
    }

}

