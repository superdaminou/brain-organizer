use egui::Ui;
use strum::IntoEnumIterator;

use crate::{application_error::ApplicationError, finance::{depense::{Depense, REPETITION}, ConnecteurDepense}};

use super::fenetre_finance::SectionFinance;


pub fn finances_gui(section: &mut SectionFinance, ui:&mut Ui) -> Result<(), ApplicationError> {

    create_depense(section, ui)?;

    ui.horizontal(|ui: &mut egui::Ui| {
        let button = egui::Button::new("Tout récuperer");

        if ui.add(button).clicked() {
            section.depenses = section.connecteur.get_all()?;
            section.calcul = calcul(&section.depenses, &section.mode_calcul);
        }
        Ok::<(), ApplicationError>(())
    }).inner?;

    table_depense(section, ui)?;

    ui.add_space(10.0);

    egui::ComboBox::from_label("Mode de calcul")
            .selected_text(format!("{:?}", section.mode_calcul.to_string()))
            .show_ui(ui, |ui| {
                REPETITION::iter().for_each(|repetition| {
                    let value = ui.selectable_value(&mut &section.mode_calcul, &repetition, repetition.to_string());
                    if value.clicked() {
                        section.mode_calcul = repetition.clone();
                        section.calcul = calcul(&section.depenses, &section.mode_calcul);
                    };
                    
                })
            }
        );

    ui.label("Total: ".to_string() + &section.calcul.to_string());

    Ok(())
}

fn table_depense(section: &mut SectionFinance, ui: &mut Ui) -> Result<(), ApplicationError> {
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
                section.connecteur.delete(&depense.id.unwrap().to_string())
                .and_then(|_|section.connecteur.get_all())?;
            }
            ui.end_row();
            Ok::<(), ApplicationError>(())
        })?;
        Ok::<(), ApplicationError>(())
    }).inner?;
    Ok(())
}



fn create_depense(section: &mut SectionFinance, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Libelle ");
        ui.text_edit_singleline(&mut section.depense.libelle);
    
        ui.label("Montant");
        ui.add(egui::DragValue::new(&mut section.depense.montant).speed(0.1));

        egui::ComboBox::from_id_salt("repetition")
            .selected_text(format!("{:?}", section.depense.repetition.to_string()))
            .show_ui(ui, |ui| {
                REPETITION::iter().for_each(|repetition| {
                    let value = ui.selectable_value(&mut &section.depense.repetition, &repetition, repetition.to_string());
                    if value.clicked() {
                        section.depense.repetition = repetition.clone();
                    };
                    
                })
            }
        );

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
            section.calcul = calcul(&section.depenses, &section.mode_calcul);   
        }

        Ok::<(), ApplicationError>(())
    }).inner?;

    Ok(())
}

fn calcul(depenses: &Vec<Depense>, mode_calcul: &REPETITION) -> f32 {
    depenses.iter()
    .map(|d| d.convert(&mode_calcul))
    .fold(0.0, |acc, x| acc+ x)

}