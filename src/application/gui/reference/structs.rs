use crate::application::{error::ApplicationError, gui::structs::Fenetre, reference::{service::get_all, structs::{reference::Reference, tag::Tag}}};

use super::reference_gui::section_references;

use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReference {
    pub reference: Reference,
    pub list_references: Vec<Reference>,
    pub tag_filter: Vec<Tag>
}


impl Default for SectionReference {
    fn default() -> Self {
        Self {
            reference: Reference::default(),
            list_references: get_all().unwrap_or_default(),
            tag_filter: vec![]
        }
    }
}


impl Fenetre for SectionReference {
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