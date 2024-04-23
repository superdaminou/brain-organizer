use strum::IntoEnumIterator;

use crate::application::{error::ApplicationError, reference::{service::{create_or_update, delete, get_all}, structs::tag::Tag}};

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
                
    list_references(section, ui);

    Ok(())
}

fn create_reference(section: &mut SectionReference, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Titre ");
        ui.text_edit_singleline(&mut section.reference.titre);

        egui::ComboBox::from_id_source("Tags").selected_text("Tags")
            .show_ui(ui, |ui| {
                Tag::iter().for_each(|t| {
                    let tag_label = ui.selectable_label(section.reference.tags.contains(&t), t.to_string());
                    if tag_label.clicked() {
                        if section.reference.tags.contains(&t) {
                            section.reference.tags.retain(|tag| !t.eq(tag));
                        } else {
                            section.reference.tags.push(t);
                        }
                    
                    }
                });
            }
        );

        ui.label("URL");
        ui.text_edit_singleline(&mut section.reference.url);

        
        let button = egui::Button::new("Enregistrer");


        if ui.add(button).clicked() {
            return create_or_update(&section.reference.clone().into())
                .and_then(|_|get_all())
                .map(|list| 
                    list.iter()
                    .map(|reference|ReferenceGui::from(reference.clone()))
                    .collect::<Vec<ReferenceGui>>())
                .map(|list| section.list_references = list)
                .map(|_| section.reference = ReferenceGui::new());
                    
        }

        Ok(())
    }).inner?;

    Ok(())
}


fn list_references (section: &mut SectionReference, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .id_source("reference")
        .max_height(300.0)
        .show(ui, |ui| {
            for contenu in &section.list_references {
                ui.horizontal(|ui| {
                    ui.label(&contenu.id.clone().unwrap_or("".to_string()));
                    ui.label(&contenu.titre);
                    ui.label(&contenu.tags.iter().map(Tag::to_string).collect::<Vec<String>>().join(", "));
                    ui.hyperlink(&contenu.url);
                    if ui.button("Modifier").clicked() {
                        section.reference = contenu.clone();
                    }
                    if ui.button("Supprimer").clicked() {
                        delete(&contenu.clone().into());
                    }
                    ui.allocate_space(ui.available_size());
                });
            }
        });
}