use crate::application::{error::ApplicationError, gui::structs::Fenetre, reference::{service, structs::{reference::Reference, tag::Tag}}};

use super::panel_gui::section_references;

use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PanelReference {
    pub reference: Reference,
    pub list_references: Vec<Reference>,
    pub tag_filter: Vec<Tag>,
    pub search: String
}

impl Default for PanelReference {
    fn default() -> Self {
        let references = service::search(&String::default(), &[]).unwrap_or_default();
        Self { 
            reference: Default::default(), 
            list_references: references, 
            tag_filter: Default::default(), 
            search: Default::default() 
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
        .scroll2(true)
        .show(ctx, |ui| {
            section_references(self, ui)
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