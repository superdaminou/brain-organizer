use log::info;

use crate::{application_error::ApplicationError, gui::{composant::EditText, Fileable}, notes::ConnecteurNote};

use super::section_note::SectionNote;
use anyhow::{Context, Result};


pub fn section_notes(section: &mut SectionNote, ui: &mut egui::Ui, ) -> Result<(), ApplicationError> {
    EditText::default().show(ui, &mut section.edit_reflexion, &section.connecteur)?;
    new_reflexion(section, ui)?;
    list_reflexions(section, ui)?;
    Ok(())
}


fn new_reflexion(section: &mut SectionNote, ui: &mut egui::Ui) -> Result<()> {
    ui.heading("Reflexion");
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Sujet");
        ui.text_edit_singleline(&mut section.reflexion.sujet);
    
        let button = egui::Button::new("Créer");
        if ui.add(button).clicked() {
            return section.connecteur.create(&section.reflexion.clone())
                .and_then(|_| section.connecteur.get_all().context("Coulnt get all"))
                .map(|result| section.list_reflexions = result);

        }
        Ok(())
    }).inner?;

    Ok(())

}


fn list_reflexions(section: &mut SectionNote, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui| {
        if ui.button("Recharger reflexion").clicked() {
            match section.connecteur.get_all() {
                Ok(result) => {
                    section.list_reflexions = result;
                },
                Err(error) => info!("Error: {}", error)
            }
        }

    });

    egui::ScrollArea::vertical()
        .id_salt("reflexion")
        .show(ui, |ui| {
            section.list_reflexions.clone().iter().try_for_each(|note| {
                ui.horizontal(|ui| {
                    ui.label(&note.sujet);
                    if ui.button("Ouvrir").clicked() {
                        section.edit.open(note, &mut section.edit_reflexion, &section.connecteur)?;
                        section.edit_reflexion.show = true;
                        return Ok::<(), ApplicationError>(());
                    }
                    if ui.button("Supprimer").clicked() {
                        return Ok(section.connecteur.delete(&note.id())
                            .and_then(|_| section.connecteur.get_all().context("get All"))
                            .map(|result| section.list_reflexions = result)?);
                    }
                    Ok::<(), ApplicationError>(())
                });
                Ok::<(), ApplicationError>(())
            })
        }).inner?;

        Ok(())
}