use egui::Ui;

use crate::application::{database::CRUD, error::ApplicationError, finance::{depense::Depense, service}};

use super::fenetre_finance::FenetreFinance;
use anyhow::Result;


pub fn finances_gui(fenetre: &mut FenetreFinance, ui:&mut Ui) -> Result<(), ApplicationError> {

    create_depense(fenetre, ui)?;

    ui.horizontal(|ui: &mut egui::Ui| {
        let button = egui::Button::new("Tout récuperer");

        if ui.add(button).clicked() {
            fenetre.depenses = Depense::get_all()?;
        }
        Ok::<(), ApplicationError>(())
    }).inner?;


    egui::Grid::new("some_unique_id").show(ui, |ui| {
        //HEADER
        ui.label("Libelle");
        ui.label("Montant");
        ui.end_row();

        fenetre.depenses.iter().try_for_each(|depense| {
            ui.label(depense.libelle.clone());
            ui.label(depense.montant.to_string());
            let supprimer = egui::Button::new("Supprimer");
            if ui.add(supprimer).clicked() {
                Depense::delete(depense)?;
            }
            ui.end_row();
            Ok::<(), ApplicationError>(())
        })?;
        Ok::<(), ApplicationError>(())
    }).inner?;


    ui.label("Total: ".to_string() + &fenetre.depenses.iter().fold(0.0, |acc, x | acc + x.montant).to_string());

    Ok(())
}



fn create_depense(section: &mut FenetreFinance, ui: &mut egui::Ui) -> Result<()> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Libelle ");
        ui.text_edit_singleline(&mut section.depense.libelle);
    
        ui.label("Montant");
        ui.add(egui::DragValue::new(&mut section.depense.montant).speed(0.1));

        let button = egui::Button::new("Enregistrer");

        if ui.add(button).clicked() {
            service::create_or_update(&section.depense)?;
            section.depense = Depense::default();
            section.depenses = Depense::get_all()?;
            
        }

        Ok::<(), anyhow::Error>(())
    }).inner?;

    Ok(())
}

