use strum::IntoEnumIterator;

use crate::application::{error::ApplicationError, reference::{service::{create_or_update, delete, filter_by_tags}, structs::{reference::Reference, tag::Tag}}};

use super::structs::SectionReference;
use anyhow::Result;


pub fn section_references(section: &mut SectionReference, ui: &mut egui::Ui) -> Result<()> {
    ui.heading("Reference");
    create_reference(section, ui)?;
    filter_bar(section, ui)?;
    ui.separator();
    list_references(section, ui)
}

fn filter_bar(section: &mut SectionReference, ui: &mut egui::Ui) -> Result<()> {
    ui.horizontal::<Result<()>>(|ui| {
        Tag::iter().try_for_each(|t| {
            let tag_label = ui.selectable_label(section.tag_filter.contains(&t), t.to_string());
            if tag_label.clicked() {
                update_tag_filter(&t, section)?;
            };
            Ok::<(), anyhow::Error>(())
        })?;
        Ok(())
    }).inner?;
    Ok(())
}

fn update_tag_filter(tag: &Tag, section: &mut SectionReference) -> Result<()>{
    if section.tag_filter.contains(tag) {
        section.tag_filter.retain(|tag| !tag.eq(tag));
    } else {
        section.tag_filter.push(tag.clone());
    }

    filter_by_tags(&section.tag_filter)
        .map(|references |section.list_references = references)
}

fn create_reference(section: &mut SectionReference, ui: &mut egui::Ui) -> Result<()> {
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
            return create_or_update(&section.reference.clone())
                .and_then(|_|filter_by_tags(&section.tag_filter))
                .map(|list| section.list_references = list)
                .map(|_| section.reference = Reference::default());
                    
        }

        Ok(())
    }).inner?;

    Ok(())
}


fn list_references (section: &mut SectionReference, ui: &mut egui::Ui) -> Result<()>{
    egui::ScrollArea::vertical()
        .id_source("reference")
        .show(ui, |ui| {
            section.list_references.iter().try_for_each(|reference| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to(&reference.titre, &reference.url);
                    ui.label(reference.tags.iter().map(Tag::to_string).collect::<Vec<String>>().join(", "));
                    if ui.button("Modifier").clicked() {
                        section.reference = reference.clone();
                    }
                    if ui.button("Supprimer").clicked() {
                        delete(reference)?;
                    }
                    ui.allocate_space(ui.available_size());
                    Ok::<(),anyhow::Error>(())
                });

                Ok::<(),ApplicationError>(())
            })
        });
    Ok(())
}