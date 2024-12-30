use search_bar::search_bar;
use uuid::Uuid;

use crate::{application_error::ApplicationError, reference::ConnecteurReference};

use super::panel::{Evenement, PanelReference};

mod search_bar;

pub fn show (section: &mut PanelReference, ui: &mut egui::Ui) -> Result<Vec<Evenement>, ApplicationError>{
    let mut evenements = Vec::default();
    ui.heading("Liste References");
    search_bar(section, ui)?;
    evenements = [evenements, liste_references(ui, section)].concat() ;
    Ok(evenements)
}

fn liste_references(ui: &mut egui::Ui, section: &mut PanelReference) -> Vec<Evenement> {
    let mut evenements : Vec<Evenement> = vec![];
    egui::ScrollArea::vertical()
        .id_salt("reference")
        .show(ui, |ui| {
            section.list_references.iter().try_for_each(|reference| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to(&reference.titre, &reference.url);
                    ui.label(reference.tags.iter().map(|t|t.0.clone()).collect::<Vec<String>>().join(", "));
                    ui.label(reference.date_creation.to_string());
                    ui.label(if reference.to_read {"Non Lu".to_string()} else {"Lu".to_string()});
                
                    if ui.button("Copier").clicked() {
                        let link_copy =  format!("[{}]({})",reference.titre, reference.url);
                        let mut clipboard = clippers::Clipboard::get();
                        clipboard.write_text(link_copy).unwrap();
                    }
                
                    if ui.button("Modifier").clicked() {
                        section.creation_reference.reference = reference.clone();
                    }
                
                    if ui.button("Supprimer").clicked() {
                        reference.id.clone().ok_or(ApplicationError::EmptyOption("id".to_string()))
                        .and_then(|id| Uuid::parse_str(&id).map_err(ApplicationError::from))
                        .map_err(ApplicationError::from)
                        .and_then(|id|section.connecteur.delete(&id));

                        evenements.push(Evenement::Reset);
                    }
                
                    ui.allocate_space(ui.available_size());
                    Ok::<(),ApplicationError>(())
                });

                Ok::<(),ApplicationError>(())
            })
        });
        
    evenements
}