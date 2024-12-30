
use crate::{application_error::ApplicationError, connecteur::Connecteur, reference::{structs::reference::Reference, ConnecteurReference, Tag}};

use super::panel::{Evenement, PanelReference};
use egui::RichText;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreationReference {
    pub reference: Reference,
    pub tag: Tag,
    pub existing_tags: Vec<Tag>,
    pub markdown_name: String,
    pub mode: Mode, 
}

impl CreationReference {
    pub fn new(connecteur: &Connecteur) -> Self {
        let tags = connecteur.all_tags_distinct().unwrap_or_default();
        Self {
            reference: Default::default(), 
            tag: Default::default(), 
            existing_tags: tags,
            markdown_name: String::default(),
            mode: Mode::Classique
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum Mode {
    Markdown,
    Classique
}

impl CreationReference {
    pub fn set_tags(&mut self, tags: Vec<Tag> ){
            self.existing_tags = tags
    }
}


pub fn show(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<Vec<Evenement>, ApplicationError> {
    let mut evenements = Vec::default();

    ui.heading("Nouvelle Reference");

    libelle_reference(ui, section)?;
    reference_tags(section, ui)?;

    ui.add_space(10.0);
    // Tags existants
    existing_tags(ui, section)?;

    ui.add_space(20.0);
    enregistrer(ui, section, &mut evenements)?;

    Ok(evenements)
}

fn enregistrer(ui: &mut egui::Ui, section: &mut PanelReference, evenements: &mut Vec<Evenement>) -> Result<(), ApplicationError> {
    let enregistrer = egui::Button::new("Enregistrer");
    if ui.add(enregistrer).clicked() {
        if section.creation_reference.mode == Mode::Markdown {
            section.creation_reference.reference.titre = section.creation_reference.markdown_name.chars()
                .skip(1)
                .take_while(|c| !c.eq_ignore_ascii_case(&']') )
                .collect::<String>();

            section.creation_reference.reference.url = section.creation_reference.markdown_name.chars()
                .skip_while(|c|!c.eq_ignore_ascii_case(&'('))
                .skip(1)
                .take_while(|c|!c.eq_ignore_ascii_case(&')'))
                .collect();
        }


        if section.creation_reference.reference.id.is_some() {
            section.connecteur.update(&section.creation_reference.reference.clone())
                .and_then(|_|section.connecteur.search(Some(&section.search), &section.filtre_tag.tags, section.filtre_tag.mode))
                .map(|list| section.list_references = list)
                .map(|_| reset(&mut section.creation_reference))?;
        } else {

            section.connecteur.create(&section.creation_reference.reference.clone())
                .and_then(|_|section.connecteur.search(Some(&section.search), &section.filtre_tag.tags, section.filtre_tag.mode))
                .map(|list| section.list_references = list)
                .map(|_| reset(&mut section.creation_reference))?;
        }
    
        evenements.push(Evenement::Reset);
            
    };
    Ok(())
}

fn existing_tags(ui: &mut egui::Ui, section: &mut PanelReference) -> Result<(), ApplicationError> {
    ui.label(RichText::new("Tag existants").strong());

    let mut adding_boutons: Vec<(egui::Response, Tag)> = vec![];

    section.creation_reference.existing_tags.chunks(10)
        .try_for_each(|chunk| {
            ui.horizontal::<Result<(), ApplicationError>>(|ui| {
                chunk.iter().try_for_each(|tag| {
                    adding_boutons.push((ui.selectable_label(false, tag.0.clone()), tag.clone()));
                    Ok::<(), ApplicationError>(())
                })?;
                Ok(())
            }).inner?;
            Ok::<(), ApplicationError>(())
        })?;

    adding_boutons.iter().try_for_each(|tag| {
        if tag.0.clicked() {
            section.creation_reference.reference.tags.insert(tag.1.clone());
        };
        Ok::<(), ApplicationError>(())
    })?;

    Ok(())
}

fn libelle_reference(ui: &mut egui::Ui, section: &mut PanelReference) -> Result<(), ApplicationError> {
    ui.horizontal(|ui: &mut egui::Ui| {
        if ui.button("Changer mode").clicked() {
            match section.creation_reference.mode {
                Mode::Classique => section.creation_reference.mode = Mode::Markdown,
                Mode::Markdown => section.creation_reference.mode = Mode::Classique,
            }
        }
    
        match section.creation_reference.mode {
            Mode::Classique => {
                ui.label("Titre: ");
                ui.text_edit_singleline(&mut section.creation_reference.reference.titre);
    
                ui.label("URL: ");
                ui.text_edit_singleline(&mut section.creation_reference.reference.url);
            },
            Mode::Markdown => {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Format Markdown: ");
                    ui.text_edit_singleline(&mut section.creation_reference.markdown_name);
                    Ok::<(), ApplicationError>(())
                }).inner?;
            },
        }
   

        ui.checkbox(&mut section.creation_reference.reference.to_read, "Non Consulté");
        Ok::<(), ApplicationError>(())
    }).inner?;
    Ok(())
}

fn reset(creation_reference: &mut CreationReference) {
    creation_reference.reference = Reference::default();
    creation_reference.markdown_name = String::default();
}

fn reference_tags(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Tag: ");
        ui.text_edit_singleline(&mut section.creation_reference.tag.0);
        if ui.add(egui::Button::new("Ajouter")).clicked() {
            section.creation_reference.reference.tags.insert(section.creation_reference.tag.clone());
            section.creation_reference.tag = Tag::default();
        }
        Ok::<(), ApplicationError>(())
    }).inner?;

    ui.horizontal(|ui: &mut egui::Ui| {
        let mut delete_boutons = vec![];
        section.creation_reference.reference.tags.iter()
            .by_ref()
            .for_each(|tag|{
                ui.label(&tag.0);
                delete_boutons.push((ui.add(egui::Button::new("Supprimer")), tag.clone()));
            });

        delete_boutons.iter().try_for_each(|tag| {
            if tag.0.clicked() {
                section.creation_reference.reference.tags.remove(&tag.1);
            };
            Ok::<(), ApplicationError>(())
        })?;
        Ok::<(), ApplicationError>(())      
    }).inner
}