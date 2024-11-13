
use list_reference::search_bar;

use crate::{application_error::ApplicationError, database::CRUD, reference::structs::reference::Reference};

use super::panel::{Evenement, PanelReference};

mod list_reference;



pub fn show (section: &mut PanelReference, ui: &mut egui::Ui) -> Result<Vec<Evenement>, ApplicationError>{
    let evenements = Vec::default();
    ui.heading("Liste References");
    search_bar(section, ui)?;


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
                    }if ui.button("Modifier").clicked() {
                        section.creation_reference.reference = reference.clone();
                    }
                    if ui.button("Supprimer").clicked() {
                        Reference::delete(reference)?;
                    }
                    
                    ui.allocate_space(ui.available_size());
                    Ok::<(),anyhow::Error>(())
                });

                Ok::<(),ApplicationError>(())
            })
        });
    Ok(evenements)
}