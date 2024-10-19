use crate::{database::CRUD, error::ApplicationError, reference::{self, structs::reference::Reference}, tag::Tag};
use anyhow::Result;
use super::panel::PanelReference;


pub fn show (section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()>{
    ui.heading("Liste References");
    filter_bar(section, ui)?;
    search_bar(section, ui)?;


    egui::ScrollArea::vertical()
        .id_salt("reference")
        .show(ui, |ui| {
            section.list_references.iter().try_for_each(|reference| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to(&reference.titre, &reference.url);
                    ui.label(reference.tags.iter().map(|t|t.0.clone()).collect::<Vec<String>>().join(", "));
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


fn filter_bar(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    let mut selectables_tag = vec![];
    ui.horizontal::<Result<()>>(|ui| {
        section.tags.iter().try_for_each(|tag| {
            selectables_tag.push((ui.selectable_label(section.tag_filter.contains(tag), tag.0.clone()), tag.clone()));
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
