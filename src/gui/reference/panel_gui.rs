use strum::IntoEnumIterator;

use crate::{database::CRUD, error::ApplicationError, reference::{self, structs::reference::Reference}, tag::Tag};

use super::panel::PanelReference;
use anyhow::Result;


pub fn section_references(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    ui.heading("Nouvelle Reference");
    create_reference(section, ui)?;
    ui.separator();
    ui.heading("Liste References");
    filter_bar(section, ui)?;
    search_bar(section, ui)?;
    list_references(section, ui)
}

fn filter_bar(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    let mut selectables_tag = vec![];
    ui.horizontal::<Result<()>>(|ui| {
        section.tags.iter().try_for_each(|tag| {
            selectables_tag.push((ui.selectable_label(section.tag_filter.contains(tag), tag.to_string()), tag.clone()));
            Ok::<(), anyhow::Error>(())
        })?;
        Ok(())
    }).inner?;

    selectables_tag.iter().try_for_each(|tag| {
        if tag.0.clicked() {
            update_tag_filter(&tag.1, section)?;
        };
        Ok::<(), anyhow::Error>(())
    })
}

fn search_bar(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    ui.horizontal::<Result<()>>(|ui| {
        ui.label("Nom ");
        ui.text_edit_singleline(&mut section.search);
        

        let button = egui::Button::new("Rechercher");

        if ui.add(button).clicked() {
            return reference::service::search(&section.search.clone(), &section.tag_filter)
                .map(|list| section.list_references = list)
                .map(|_| section.creation_reference.reference = Reference::default());
                    
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
        ui.label("Titre: ");
        ui.text_edit_singleline(&mut section.creation_reference.reference.titre);

        ui.label("URL: ");
        ui.text_edit_singleline(&mut section.creation_reference.reference.url);

        ui.checkbox(&mut section.creation_reference.reference.to_read, "Non Consulté");
        Ok::<(), anyhow::Error>(())
    }).inner?;

    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Tag: ");
        ui.text_edit_singleline(&mut section.creation_reference.tag);
        if ui.add(egui::Button::new("Ajouter")).clicked() {
            section.creation_reference.reference.tags.insert(section.creation_reference.tag.clone());
            section.creation_reference.tag = "".to_string();
        }
        Ok::<(), anyhow::Error>(())
    });

    ui.horizontal(|ui: &mut egui::Ui| {
        let mut delete_boutons = vec![];
        section.creation_reference.reference.tags.iter()
            .by_ref()
            .for_each(|tag|{
                ui.label(tag);
                delete_boutons.push((ui.add(egui::Button::new("Supprimer")), tag.clone()));
            });

        delete_boutons.iter().try_for_each(|tag| {
            if tag.0.clicked() {
                section.creation_reference.reference.tags.remove(&tag.1);
            };
            Ok::<(), anyhow::Error>(())
        })?;
        Ok::<(), anyhow::Error>(())      
    });

    ui.add_space(5.0);
    // Tags existants
    ui.label("Tag existants");
    ui.horizontal(|ui: &mut egui::Ui| {
        let mut adding_boutons = vec![];
        section.creation_reference.existing_tags.iter().for_each(|tag|{
            adding_boutons.push((ui.add(egui::Button::new(tag)), tag.clone()));
        });

        adding_boutons.iter().try_for_each(|tag| {
            if tag.0.clicked() {
                section.creation_reference.reference.tags.insert(tag.1.clone());
            };
            Ok::<(), anyhow::Error>(())
        })?;
        Ok::<(), anyhow::Error>(())  
    }).inner?;

    ui.add_space(20.0);
    let enregistrer = egui::Button::new("Enregistrer");

    if ui.add(enregistrer).clicked() {
        return reference::service::create_or_update(&section.creation_reference.reference.clone())
            .and_then(|_|reference::service::search(&section.search, &section.tag_filter))
            .map(|list| section.list_references = list)
            .map(|_| section.creation_reference.reference = Reference::default());
                
    }

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
                        let link_copy =  format!("[{}]({})",reference.titre, reference.url);
                        let mut clipboard = clippers::Clipboard::get();
                        clipboard.write_text(link_copy).unwrap();
                    }if ui.button("Modifier").clicked() {
                        section.creation_reference.reference = reference.clone();
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