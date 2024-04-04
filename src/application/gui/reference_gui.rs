

use serde::{Deserialize, Serialize};

use crate::application::reference::{reference::{create, delete, get_all, Reference, Tag}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReferenceGui {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub categorie: String
}


impl From<Reference> for ReferenceGui {
    fn from(value: Reference) -> Self {
        ReferenceGui {
            id: value.id,
            titre: value.titre,
            url: value.url,
            categorie: value.categorie.join(",")
        }
    }
}

impl From<ReferenceGui> for Reference {
    fn from(val: ReferenceGui) -> Self {
        Reference {
            id: val.id,
            titre: val.titre,
            url: val.url,
            categorie: val.categorie.split(',').map(String::from).collect::<Vec<Tag>>()
        }
    }
}




pub fn section_references<'a>(section: &mut SectionReference, ui: &mut egui::Ui) {
    
    ui.heading("Reference");

    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Titre ");
        ui.text_edit_singleline(&mut section.reference.titre);

        ui.label("Categorie");
        ui.text_edit_singleline(&mut section.reference.categorie);

        ui.label("URL");
        ui.text_edit_singleline(&mut section.reference.url);

        let button = egui::Button::new("Enregistrer");


        if ui.add(button).clicked() {
            match create(&section.reference.clone().into()) {
                Ok(_result) => println!("Inserted"),
                Err(error) => println!("Error: {}", error)
            }
            match get_all() {
                Ok(result) => {
                    section.list_references = result.iter().map(|result |ReferenceGui::from(result.clone())).collect::<Vec<ReferenceGui>>();
                },
                Err(error) => println!("Error: {}", error)
            }
        }
    });

    ui.horizontal(|ui| {
        if ui.button("Recharger reference").clicked() {
            match get_all() {
                Ok(result) => {
                    section.list_references = result.iter().map(|result |ReferenceGui::from(result.clone())).collect::<Vec<ReferenceGui>>();
                },
                Err(error) => println!("Error: {}", error)
            }
        }

    });
                
    egui::ScrollArea::vertical()
        .id_source("reference")
        .max_height(300.0)
        .show(ui, |ui| {
            for contenu in &section.list_references {
                ui.horizontal(|ui| {
                    ui.label(&contenu.id.clone().unwrap_or("".to_string()));
                    ui.label(&contenu.titre);
                    ui.label(&contenu.categorie);
                    ui.hyperlink(&contenu.url);
                    if ui.button("Supprimer").clicked() {
                        match delete(&contenu.clone().into()) {
                            Ok(_) => println!("Deleted"),
                            Err(error) => println!("Error: {}", error)
                        }
                    }
                });
            }
        });
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReference {
    pub reference: ReferenceGui,
    pub list_references: Vec<ReferenceGui>,
}

impl SectionReference {
    pub fn new() -> Self {
        Self {
            reference: Reference { id: None, titre: "titre".to_string(), url: "String".to_string(), categorie: vec![] }.into(),
            list_references: get_all().unwrap_or_default().iter().map(|reference| ReferenceGui::from(reference.clone())).collect::<Vec<ReferenceGui>>()
        }
    } 
}