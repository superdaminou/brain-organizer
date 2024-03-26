use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use super::error::ApplicationError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Evenement {
    pub id: Option<u32>,
    pub titre: String,
    pub niveau: String
}



pub fn create(connection: &Connection, contenu: &Evenement) -> Result<usize, ApplicationError> {
    return connection.execute(
        "INSERT INTO evenement (titre, niveau) VALUES (?1, ?2)",
        (contenu.titre.clone(), contenu.niveau.clone()),
    ).map_err(ApplicationError::from);
}


pub fn get_all(connection: &Connection) -> Result<Vec<Evenement>, ApplicationError> {
    let mut stmt = connection.prepare("SELECT id, titre, niveau FROM evenement;").map_err(ApplicationError::from)?;
    return Ok(stmt.query_map([], |row| {
            Ok(Evenement {
                id: row.get(0)?,
                titre: row.get(1)?,
                niveau: row.get(2)?,
            })
        })?.map(|f| f.unwrap()).collect::<Vec<Evenement>>());

}


pub fn section_evenements(mut evenements: &mut &Vec<Evenement>, evenement: &mut Evenement, ui: &mut egui::Ui, connection: &Connection) -> Result<(), ApplicationError> {
    ui.heading("Evenements");
    ui.horizontal(|ui| {
        ui.label("Titre ");
        ui.text_edit_singleline(&mut evenement.titre);

        ui.label("Niveau");
        ui.text_edit_singleline(&mut evenement.niveau)
    });

    let mut events = Vec::new() ;
    if ui.button("Enregistrer évenement").clicked() {

        events = create(&connection, &evenement)
        .and_then(|_| get_all(&connection))?;

    }

    egui::ScrollArea::vertical().id_source("list_evenements").max_height(5.0).show(ui, |ui| {
        for evenement in events {
            ui.horizontal(|ui| {
                ui.label(&evenement.titre);
            });
        }
    });

    return Ok(());
}
