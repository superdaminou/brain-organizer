use egui::Ui;

use crate::{application_error::ApplicationError, database::CRUD, finance::{depense::Depense, ConnecteurDepense}};

use super::fenetre_finance::SectionFinance;


pub fn finances_gui(section: &mut SectionFinance, ui:&mut Ui) -> Result<(), ApplicationError> {

    create_depense(section, ui)?;

    ui.horizontal(|ui: &mut egui::Ui| {
        let button = egui::Button::new("Tout récuperer");

        if ui.add(button).clicked() {
            section.depenses = section.connecteur.get_all()?;
        }
        Ok::<(), ApplicationError>(())
    }).inner?;


    egui::Grid::new("depenses").show(ui, |ui| {
        ui.label("Libelle");
        ui.label("Montant");
        ui.label("Repetition");
        ui.end_row();

        section.depenses.iter().try_for_each(|depense| {
            ui.label(depense.libelle.clone());
            ui.label(depense.montant.to_string());
            ui.label(depense.repetition.to_string());

            let modifier = egui::Button::new("Modifier");
            if ui.add(modifier).clicked() {
                section.depense = depense.clone();
            }



            let supprimer = egui::Button::new("Supprimer");
            if ui.add(supprimer).clicked() {
                section.connecteur.delete(&depense.id.unwrap().to_string())?;
            }
            ui.end_row();
            Ok::<(), ApplicationError>(())
        })?;
        Ok::<(), ApplicationError>(())
    }).inner?;


    ui.label("Total: ".to_string() + &section.depenses.iter().fold(0.0, |acc, x | acc + x.montant).to_string());

    Ok(())
}



fn create_depense(section: &mut SectionFinance, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Libelle ");
        ui.text_edit_singleline(&mut section.depense.libelle);
    
        ui.label("Montant");
        ui.add(egui::DragValue::new(&mut section.depense.montant).speed(0.1));

        let button = egui::Button::new("Enregistrer");

        if ui.add(button).clicked() {

            if section.depense.id.is_some() {
                section.connecteur.update(&section.depense.clone())
                    .and_then(|_|section.connecteur.get_all())
                    .map(|list| section.depenses = list)?
            } else {
    
                section.connecteur.create(&section.depense.clone())
                    .and_then(|_|section.connecteur.get_all())
                    .map(|list| section.depenses = list)?
            }

            section.depense = Depense::default();
            section.depenses = section.connecteur.get_all()?;
            
        }

        Ok::<(), ApplicationError>(())
    }).inner?;

    Ok(())
}

