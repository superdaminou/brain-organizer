
use crate::application::{error::ApplicationError, reference::service::{create, delete, get_all}};

use super::structs::{ReferenceGui, SectionReference};


pub fn section_references(section: &mut SectionReference, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.heading("Reference");

    create_reference(section, ui)?;

    ui.horizontal(|ui| {
        if ui.button("Recharger reference").clicked() {
            return get_all()
                .map(|list| 
                    list.iter()
                    .map(|reference|ReferenceGui::from(reference.clone()))
                    .collect::<Vec<ReferenceGui>>())
                .map(|list| section.list_references = list);
        }

        Ok(())

    }).inner?;
                
    list_references(&mut section.list_references, ui);

    Ok(())
}

fn create_reference(section: &mut SectionReference, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Titre ");
        ui.text_edit_singleline(&mut section.reference.titre);

        ui.label("Categorie");
        ui.text_edit_singleline(&mut section.reference.categorie);

        ui.label("URL");
        ui.text_edit_singleline(&mut section.reference.url);

        
        let button = egui::Button::new("Enregistrer");


        if ui.add(button).clicked() {
            return create(&section.reference.clone().into())
                .and_then(|_|get_all())
                .map(|list| 
                    list.iter()
                    .map(|reference|ReferenceGui::from(reference.clone()))
                    .collect::<Vec<ReferenceGui>>())
                .map(|list| section.list_references = list);
        }

        Ok(())
    }).inner?;

    Ok(())
}


fn list_references (list_references: &mut Vec<ReferenceGui>, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .id_source("reference")
        .max_height(300.0)
        .show(ui, |ui| {
            for contenu in list_references {
                ui.horizontal(|ui| {
                    ui.label(&contenu.id.clone().unwrap_or("".to_string()));
                    ui.label(&contenu.titre);
                    ui.label(&contenu.categorie);
                    ui.hyperlink(&contenu.url);
                    if ui.button("Supprimer").clicked() {
                        delete(&contenu.clone().into());
                    }
                    ui.allocate_space(ui.available_size());
                
                });
            }
        });
}