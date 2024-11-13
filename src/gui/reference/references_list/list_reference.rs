use crate::{gui::reference::panel::PanelReference, reference::{self, structs::reference::Reference, tag::Tag, ModeTags}};
use anyhow::Result;


pub fn search_bar(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {    
    ui.horizontal::<Result<()>>(|ui| {
        ui.label("Nom ");
        ui.text_edit_singleline(&mut section.search);
        
        let button = egui::Button::new("Rechercher");

        if ui.add(button).clicked() {
            return reference::service::search(&section.search.clone(), &section.filtre_tag.tags, section.filtre_tag.mode)
                .map(|list| section.list_references = list)
                .map(|_| section.creation_reference.reference = Reference::default());
                    
        }
        Ok(())
    }).inner?;

    filter_tags(section, ui)?;
    Ok(())
}

pub fn update_tag_filter(tag: &Tag, section: &mut PanelReference) -> Result<()>{
    if section.filtre_tag.tags.contains(tag) {
        section.filtre_tag.tags.retain(|tag| !tag.eq(tag));
    } else {
        section.filtre_tag.tags.insert(tag.clone());
    }

    reference::service::search(&section.search, &section.filtre_tag.tags, section.filtre_tag.mode)
        .map(|references |section.list_references = references)
}


pub fn filter_tags(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    let mut selectables_tag = vec![];
    
    let button = egui::Button::new(section.filtre_tag.mode.to_string());

    if ui.add(button).clicked() {
        section.filtre_tag.mode = match section.filtre_tag.mode {
            ModeTags::INCLUS => ModeTags::EXCLUS,
            ModeTags::EXCLUS => ModeTags::INCLUS,
        };

        return reference::service::search(&section.search.clone(), &section.filtre_tag.tags, section.filtre_tag.mode)
                .map(|list| section.list_references = list)
                .map(|_| section.creation_reference.reference = Reference::default());
                
    }
    
    
    ui.horizontal::<Result<()>>(|ui| {
        section.tags.iter().try_for_each(|tag| {
            selectables_tag.push((ui.selectable_label(section.filtre_tag.tags.contains(tag), tag.0.clone()), tag.clone()));
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
