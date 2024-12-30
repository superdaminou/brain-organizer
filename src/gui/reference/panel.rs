use std::collections::HashSet;

use crate::{application_error::ApplicationError, connecteur::Connecteur, gui::structs::Fenetre, reference::{self, structs::reference::Reference,  ConnecteurReference, ModeTags, Tag}};

use super::{create_reference::{self, CreationReference, Mode}, references_list};

pub struct PanelReference {
    pub connecteur: Connecteur,
    pub creation_reference: CreationReference,
    pub list_references: Vec<Reference>,
    pub filtre_tag: FiltreTag,
    pub search: String,
    pub tags: Vec<Tag>,
    pub evenements: Vec<Evenement> 
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
pub enum Evenement {
    Reset,
    Modifier(Reference)
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct FiltreTag {
    pub tags: HashSet<Tag>,
    pub mode: ModeTags
}

impl PanelReference {
    pub fn new(connecteur: Connecteur) -> Self {
        let references = connecteur.search(None, &HashSet::default(), reference::ModeTags::OUVERT).unwrap_or_default();
        let tags = &mut connecteur.all_tags_distinct().unwrap_or_default();
        tags.sort();
        let mut creation_ref = CreationReference::new(&connecteur);
        creation_ref.set_tags(tags.clone());
        Self { 
            connecteur,
            creation_reference: creation_ref, 
            list_references: references, 
            filtre_tag: Default::default(), 
            search: Default::default(),
            tags: tags.to_owned(),
            evenements: Vec::default()
        }
    }
}


impl Fenetre for PanelReference {
    fn name(&self) -> &'static str {
        "References"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        let visible =  egui::Window::new(self.name())
        .open(is_open)
        .scroll(true)
        .show(ctx, |ui| {
            self.evenements =  section_references(self, ui)?;
            Ok::<_,ApplicationError>(())
        });

        self.evenements.iter().try_for_each(|e| {
            match e {
                Evenement::Reset => {
                    self.creation_reference.reference = Reference::default();
                    let tags = self.connecteur.all_tags_distinct()?;
                    self.tags  = tags.clone();
                    self.creation_reference.existing_tags = tags;
                },
                Evenement::Modifier(reference) => {
                    self.creation_reference.reference = reference.clone();
                    self.creation_reference.mode = Mode::Classique;
                },
            }
            Ok::<(), ApplicationError>(())
        })?;

        match visible {
            Some(windows) => {
                windows.inner.transpose()?;
                Ok(())
            },
            None => Ok(())
        }
    }
}

fn section_references(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<Vec<Evenement>, ApplicationError> {
    let mut evenements  = create_reference::show(section, ui)?;
    ui.separator();
    evenements.append(&mut references_list::show(section, ui)?);
    Ok(evenements)
}