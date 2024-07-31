use log::info;
use crate::application::{gui::composant::EditText, reflexion::{service::ReflexionDatabase, Reflexion}};

use super::section_reflexion::{SectionReflexion};
use anyhow::{Context, Result};


pub fn section_reflexions(section: &mut SectionReflexion, ui: &mut egui::Ui) -> Result<()> {
    EditText::default().show(ui, &mut section.edit_reflexion)?;
    new_reflexion(section, ui)?;
    list_reflexions(section, ui)?;
    Ok(())
}


fn new_reflexion(section: &mut SectionReflexion, ui: &mut egui::Ui) -> Result<()> {
    ui.heading("Reflexion");
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Sujet");
        ui.text_edit_singleline(&mut section.reflexion.sujet);
    
        let button = egui::Button::new("Créer");
        if ui.add(button).clicked() {
            return Reflexion::create(&section.reflexion.clone())
                .and_then(|_| Reflexion::get_all().context("Coulnt get all"))
                .map(|result| section.list_reflexions = result);

        }
        Ok(())
    }).inner?;

    Ok(())

}


fn list_reflexions(section: &mut SectionReflexion, ui: &mut egui::Ui) -> Result<()> {

    ui.horizontal(|ui| {
        if ui.button("Recharger reflexion").clicked() {
            match Reflexion::get_all() {
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
                        section.edit.open(reflexion.filename().clone(), &mut section.edit_reflexion);
                        section.edit_reflexion.show = true;
                        return Ok(());
                    }
                    if ui.button("Supprimer").clicked() {
                        return Reflexion::delete(&reflexion.clone())
                            .and_then(|_| Reflexion::get_all().context("get All"))
                            .map(|result| section.list_reflexions = result);
                    }
                    Ok(())
                });
            }
        });

        Ok(())
}