

use egui::text::LayoutJob;

use crate::application::reflexion::service::{create, delete, get_all};

use super::structs::{ReflexionGui, SectionReflexion};





pub fn section_reflexions<'a>(section: &mut SectionReflexion, ui: &mut egui::Ui) {
    
    ui.heading("Reflexion");

    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Sujet");
        ui.text_edit_singleline(&mut section.reflexion.sujet);

        ui.label("Contenu");
        ui.text_edit_multiline(&mut section.reflexion.contenu);

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                let mut job = LayoutJob::single_section(
                    "text".to_owned(),
                    egui::TextFormat {
                        ..Default::default()
                    },
                );
                job.wrap = egui::text::TextWrapping {
                    ..Default::default()
                };

                // NOTE: `Label` overrides some of the wrapping settings, e.g. wrap width
                ui.label(job);
            });

        

        let button = egui::Button::new("Enregistrer");


        if ui.add(button).clicked() {
            match create(&section.reflexion.clone().into()) {
                Ok(_result) => println!("Inserted"),
                Err(error) => println!("Error: {}", error)
            }
            match get_all() {
                Ok(result) => {
                    section.list_reflexions = result.iter().map(|result |ReflexionGui::from(result.clone())).collect::<Vec<ReflexionGui>>();
                },
                Err(error) => println!("Error: {}", error)
            }
        }
    });

    ui.horizontal(|ui| {
        if ui.button("Recharger reflexion").clicked() {
            match get_all() {
                Ok(result) => {
                    section.list_reflexions = result.iter().map(|result |ReflexionGui::from(result.clone())).collect::<Vec<ReflexionGui>>();
                },
                Err(error) => println!("Error: {}", error)
            }
        }

    });
                
    egui::ScrollArea::vertical()
        .id_source("reflexion")
        .max_height(300.0)
        .show(ui, |ui| {
            for contenu in &section.list_reflexions {
                ui.horizontal(|ui| {
                    ui.label(&contenu.id.clone().unwrap_or("".to_string()));
                    ui.label(&contenu.sujet);
                    ui.label(&contenu.contenu);
                    if ui.button("Supprimer").clicked() {
                        match delete(&contenu.clone().into()) {
                            Ok(_) => println!("Deleted"),
                            Err(error) => println!("Error: {}", error)
                        }
                    }
                });
            }
        });

        ui.allocate_space(ui.available_size());
}


