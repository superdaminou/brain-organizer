use std::{fs::{read_to_string, File}, io::Write};
use egui::{TextEdit, Ui, Window};
use crate::application::{error::ApplicationError, reflexion::{service::{create, delete, get_all}, structs::Reflexion}};
use super::structs::{EditReflexion, EditText, SectionReflexion};


pub fn section_reflexions<'a>(section: &mut SectionReflexion, ui: &mut egui::Ui) {
    EditText::default().show(ui, &mut section.edit_reflexion);
    
    ui.heading("Reflexion");
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Sujet");
        ui.text_edit_singleline(&mut section.reflexion.sujet);
    

        let button = egui::Button::new("Créer");
        if ui.add(button).clicked() {
            match create(&section.reflexion.clone().into()) {
                Ok(_result) => println!("Inserted"),
                Err(error) => println!("Error: {}", error)
            }
            match get_all() {
                Ok(result) => {
                    section.list_reflexions = result;
                },
                Err(error) => println!("Error: {}", error)
            }
        }
    });

    ui.horizontal(|ui| {
        if ui.button("Recharger reflexion").clicked() {
            match get_all() {
                Ok(result) => {
                    section.list_reflexions = result;
                },
                Err(error) => println!("Error: {}", error)
            }
        }

    });

    egui::ScrollArea::vertical()
        .id_source("reflexion")
        .max_height(300.0)
        .show(ui, |ui| {
            for reflexion in &section.list_reflexions {
                ui.horizontal(|ui| {
                    ui.label(&reflexion.id.clone().unwrap_or("".to_string()));
                    ui.label(&reflexion.sujet);
                    if ui.button("Ouvrir").clicked() {
                        section.edit.open(reflexion.clone(), &mut section.edit_reflexion);
                        section.edit_reflexion.show = true;
                    }
                    if ui.button("Supprimer").clicked() {
                        match delete(&reflexion.clone().into()) {
                            Ok(_) => println!("Deleted"),
                            Err(error) => println!("Error: {}", error)
                        }
                    }
                });
            }
        });
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
                        Err(e) => println!("Error while writing file {} :  {}", path, e.to_string()),
                        Ok(_) => println!("")
                    }
                } 
            });
            return Ok(());
    }

    pub fn open(&mut self ,reflexion: Reflexion, edit_reflexion:&mut EditReflexion) {
        println!("Opening: {}", reflexion.get_path());
        edit_reflexion.contenu = read_to_string( &reflexion.get_path()).unwrap();
        println!("Contenu: {}", edit_reflexion.contenu);
        edit_reflexion.show = !edit_reflexion.show;
        edit_reflexion.reflexion = reflexion;
    }

}