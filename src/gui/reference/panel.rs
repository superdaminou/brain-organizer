use std::collections::HashSet;

use crate::{error::ApplicationError, gui::structs::Fenetre, reference::{self, structs::reference::Reference}, tag::{self, Tag}};

use super::{create_reference::{self, CreationReference}, list_reference};

use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PanelReference {
    pub creation_reference: CreationReference,
    pub list_references: Vec<Reference>,
    pub tag_filter: Vec<Tag>,
    pub search: String,
    pub tags: Vec<Tag>,
    evenements: HashSet<Evenement> 
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
enum Evenement {
    Reset,
    Modifier(Reference)
}

impl Default for PanelReference {
    fn default() -> Self {
        let references = reference::service::search(&String::default(), &[]).unwrap_or_default();
        let tags =  tag::service::get_all_distinct().unwrap_or_default();
        Self { 
            creation_reference: Default::default(), 
            list_references: references, 
            tag_filter: Default::default(), 
            search: Default::default(),
            tags,
            evenements: HashSet::default()
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
            section_references(self, ui)
        });

        self.evenements.iter().for_each(|e| {
            match e {
                Evenement::Reset => self.creation_reference.reference = Reference::default(),
                Evenement::Modifier(reference) => self.creation_reference.reference = reference.clone(),
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

fn section_references(section: &mut PanelReference, ui: &mut egui::Ui) -> Result<()> {
    create_reference::show(section, ui)?;
    ui.separator();
    list_reference::show(section, ui)
}

