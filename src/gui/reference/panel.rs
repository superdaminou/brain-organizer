use std::collections::BTreeSet;

use crate::{application_error::ApplicationError, gui::structs::Fenetre, reference::{self, structs::reference::Reference}, tag::{self, Tag}};

use super::{create_reference::{self, CreationReference, Mode}, list_reference};

use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PanelReference {
    pub creation_reference: CreationReference,
    pub list_references: Vec<Reference>,
    pub filtre_tag: Vec<Tag>,
    pub search: String,
    pub tags: Vec<Tag>,
    pub evenements: BTreeSet<Evenement> 
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Evenement {
    Reset,
    Modifier(Reference)
}

impl Default for PanelReference {
    fn default() -> Self {
        let references = reference::service::search(&String::default(), &[]).unwrap_or_default();
        let tags =  tag::service::get_all_distinct().unwrap_or_default();
        let mut creation_ref = CreationReference::default();
        creation_ref.set_tags(tags.clone());
        Self { 
            creation_reference: creation_ref, 
            list_references: references, 
            filtre_tag: Default::default(), 
            search: Default::default(),
            tags,
            evenements: BTreeSet::default()
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

        self.evenements.iter().for_each(|e| {
            match e {
                Evenement::Reset => {
                    self.creation_reference.reference = Reference::default();
                    let tags = tag::service::get_all_distinct().unwrap_or_default();
                    self.tags  = tags.clone();
                    self.creation_reference.existing_tags = tags;
                },
                Evenement::Modifier(reference) => {
                    self.creation_reference.reference = reference.clone();
                    self.creation_reference.mode = Mode::Classique;
                },
            }
        });

        match visible {
            Some(windows) => {
                windows.inner.transpose()?;
                Ok(())
            },
            None => Ok(())
        }
    }
}

fn section_references(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<BTreeSet<Evenement>, ApplicationError> {
    let mut evenements  = create_reference::show(section, ui)?;
    ui.separator();
    evenements.extend(list_reference::show(section, ui)?);
    Ok(evenements)
}