use crate::{error::ApplicationError, gui::structs::Fenetre, reference::{self, structs::reference::Reference}, tag::{self, Tag}};

use super::panel_gui::section_references;

use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PanelReference {
    pub creation_reference: CreationReference,
    pub list_references: Vec<Reference>,
    pub tag_filter: Vec<Tag>,
    pub search: String,
    pub tags: Vec<Tag>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreationReference {
    pub reference: Reference,
    pub tag: Tag,
    pub existing_tags: Vec<Tag>
}

impl Default for CreationReference {
    fn default() -> Self {
        tag::service::get_all_distinct().unwrap_or_default();
        Self { reference: 
            Default::default(), 
            tag: Default::default(), 
            existing_tags: tag::service::get_all_distinct().unwrap_or_default() 
        }
    }
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
            tags
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