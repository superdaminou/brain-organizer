use strum::IntoEnumIterator;

use crate::application::{database::CRUD, error::ApplicationError, reference::{self, structs::{reference::Reference, tag::Tag}}};

use super::panel::PanelReference;
use anyhow::Result;


pub fn section_references(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    ui.heading("Reference");
    create_reference(section, ui)?;
    filter_bar(section, ui)?;
    search_bar(section, ui)?;
    ui.separator();
    list_references(section, ui)
}

fn filter_bar(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
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

fn search_bar(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    ui.horizontal::<Result<()>>(|ui| {
        ui.label("Nom ");
        ui.text_edit_singleline(&mut section.search);
        

        let button = egui::Button::new("Rechercher");

        if ui.add(button).clicked() {
            return reference::service::search(&section.search.clone(), &section.tag_filter)
                .map(|list| section.list_references = list)
                .map(|_| section.reference = Reference::default());
                    
        }
        Ok(())
    }).inner?;

    
    Ok(())
}

fn update_tag_filter(tag: &Tag, section: &mut PanelReference) -> Result<()>{
    if section.tag_filter.contains(tag) {
        section.tag_filter.retain(|tag| !tag.eq(tag));
    } else {
        section.tag_filter.push(tag.clone());
    }

    reference::service::search(&section.search, &section.tag_filter)
        .map(|references |section.list_references = references)
}

fn create_reference(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
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

        ui.checkbox(&mut section.reference.to_read, "Non Consulté");

        let button = egui::Button::new("Enregistrer");

        if ui.add(button).clicked() {
            return reference::service::create_or_update(&section.reference.clone())
                .and_then(|_|reference::service::search(&section.search, &section.tag_filter))
                .map(|list| section.list_references = list)
                .map(|_| section.reference = Reference::default());
                    
        }

        Ok(())
    }).inner?;

    Ok(())
}


fn list_references (section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()>{
    egui::ScrollArea::vertical()
        .id_source("reference")
        .show(ui, |ui| {
            section.list_references.iter().try_for_each(|reference| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to(&reference.titre, &reference.url);
                    ui.label(reference.tags.iter().map(Tag::to_string).collect::<Vec<String>>().join(", "));
                    ui.label(reference.date_creation.to_string());
                    ui.label(if reference.to_read {"Non Lu".to_string()} else {"Lu".to_string()});
                    
                    if ui.button("Copier").clicked() {
                        let link_copy =  format!("[{}]({})",reference.titre.to_string(), reference.url);
                        let mut clipboard = clippers::Clipboard::get();
                        clipboard.write_text(link_copy).unwrap();
                    }if ui.button("Modifier").clicked() {
                        section.reference = reference.clone();
                    }
                    if ui.button("Supprimer").clicked() {
                        Reference::delete(reference)?;
                    }
                    
                    ui.allocate_space(ui.available_size());
                    Ok::<(),anyhow::Error>(())
                });

                Ok::<(),ApplicationError>(())
            })
        });
    Ok(())
}