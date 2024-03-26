use eframe::App;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use super::{reference, database, evenement};

use super::{app::SectionReference, error::ApplicationError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub id: Option<u32>,
    pub titre: String,
    pub url: String,
    pub categorie: String
}

pub fn create(connection: &Connection, contenu: &Reference) -> Result<usize, ApplicationError> {
    return connection.execute(
        "INSERT INTO reference (nom, url, categorie) VALUES (?1, ?2, ?3)", (contenu.titre.clone(), contenu.url.clone(), contenu.categorie.clone()))
        .map_err(ApplicationError::from);
}


pub fn supprimer(contenu: &Reference) -> Result<usize, ApplicationError> {
     return contenu.id
        .ok_or(ApplicationError::from("Pas d'id".to_string()))
        .and_then(|id| database::opening_database().map_err(ApplicationError::from))
        .and_then(|connexion| connexion.execute("DELETE FROM reference WHERE id=?1", [contenu.id]).map_err(ApplicationError::from));
}


pub fn get_all(connection: &Connection) -> Result<Vec<Reference>, ApplicationError> {
    let mut stmt = connection.prepare("SELECT id, nom, url, categorie FROM reference;").map_err(ApplicationError::from)?;
    return Ok(stmt.query_map([], |row| {
            Ok(Reference {
                id: row.get(0)?,
                titre: row.get(1)?,
                url: row.get(2)?,
                categorie: row.get(3)?,
            })
        })?.map(|f| f.unwrap()).collect::<Vec<Reference>>());

}


pub fn section_references<'a>(section: &mut SectionReference, ui: &mut egui::Ui, connection: &Connection) {
    
    ui.heading("Reference");

    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Titre ");
        ui.text_edit_singleline(&mut section.reference.titre);

        ui.label("Categorie");
        ui.text_edit_singleline(&mut section.reference.categorie);

        ui.label("URL");
        ui.text_edit_singleline(&mut section.reference.url);
    });

    ui.horizontal(|ui| {
        if ui.button("Recharger references").clicked() {
            match get_all(&connection) {
                Ok(result) => {
                    section.list_references = result.clone();
                },
                Err(error) => println!("Error: {}", error.to_string())
            }
        }

        if ui.button("Enregistrer reference").clicked() {
            match create(&connection, &section.reference) {
                Ok(result) => println!("Inserted"),
                Err(error) => println!("Error: {}", error.to_string())
            }
        }

    });

                
    egui::ScrollArea::vertical()
        .id_source("reference")
        .max_height(5.0)
        .show(ui, |ui| {
            for contenu in &section.list_references {
                ui.horizontal(|ui| {
                    ui.label(&contenu.titre);
                    ui.label(&contenu.categorie);
                    ui.label(&contenu.url);
                    if ui.button("Enregistrer reference").clicked() {
                        match supprimer(&section.reference) {
                            Ok(result) => println!("Deleted"),
                            Err(error) => println!("Error: {}", error.to_string())
                        }
                    }
                });
            }
        });
}