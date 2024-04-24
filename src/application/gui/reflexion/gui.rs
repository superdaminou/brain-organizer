use log::info;
use crate::application::{error::ApplicationError, reflexion::service::{create, delete, get_all}};
use super::structs::{EditText, SectionReflexion};


pub fn section_reflexions(section: &mut SectionReflexion, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    EditText::default().show(ui, &mut section.edit_reflexion)?;
    new_reflexion(section, ui)?;
    list_reflexions(section, ui)?;
    Ok(())
}


fn new_reflexion(section: &mut SectionReflexion, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.heading("Reflexion");
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Sujet");
        ui.text_edit_singleline(&mut section.reflexion.sujet);
    
        let button = egui::Button::new("Créer");
        if ui.add(button).clicked() {
            return create(&section.reflexion.clone())
                .and_then(|_| get_all())
                .map(|result| section.list_reflexions = result);

        }
        Ok(())
    }).inner?;

    Ok(())

}


fn list_reflexions(section: &mut SectionReflexion, ui: &mut egui::Ui) -> Result<(), ApplicationError> {

    ui.horizontal(|ui| {
        if ui.button("Recharger reflexion").clicked() {
            match get_all() {
                Ok(result) => {
                    section.list_reflexions = result;
                },
                Err(error) => info!("Error: {}", error)
            }
        }

    });

    egui::ScrollArea::vertical()
        .id_source("reflexion")
        .show(ui, |ui| {
            for reflexion in &section.list_reflexions.clone() {
                ui.horizontal(|ui| {
                    ui.label(&reflexion.sujet);
                    if ui.button("Ouvrir").clicked() {
                        section.edit.open(reflexion.clone(), &mut section.edit_reflexion);
                        section.edit_reflexion.show = true;
                        return Ok(());
                    }
                    if ui.button("Supprimer").clicked() {
                        return delete(&reflexion.clone())
                            .and_then(|_| get_all())
                            .map(|result| section.list_reflexions = result);
                    }
                    Ok(())
                });
            }
        });

        Ok(())
}