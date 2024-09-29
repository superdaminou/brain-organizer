

use crate::{error::ApplicationError, gui::{composant::{EditFile, EditText}, structs::Fenetre}, reflexion::Reflexion};

use super::gui::section_reflexions;
use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct SectionReflexion {
    pub reflexion: Reflexion,
    pub list_reflexions: Vec<Reflexion>,
    pub edit: EditText,
    pub edit_reflexion: EditFile
}


impl Fenetre for SectionReflexion {
    fn name(&self) -> &'static str {
        "Reflexions"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        egui::Window::new(self.name())
        .open(is_open)
        .scroll2(true)
        .show(ctx, |ui| {
            section_reflexions(self, ui)
        });
        Ok(())
    }
}