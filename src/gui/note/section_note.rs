

use crate::{application_error::ApplicationError, connecteur::Connecteur, gui::{composant::{EditFileable, EditText}, structs::Fenetre}, notes::Note};

use super::gui::section_notes;
use anyhow::Result;
use log::warn;

pub struct SectionNote {
    pub connecteur: Connecteur,
    pub reflexion: Note,
    pub list_reflexions: Vec<Note>,
    pub edit: EditText,
    pub edit_reflexion: EditFileable<Note>
}

impl Default for SectionNote {
    fn default() -> Self {
        let mode_connecteur = std::env::var("MODE")
            .map(|v|Connecteur::from_str(&v))
            .unwrap_or_else(|e| {
                warn!("Erreurs lors de la lecture du mode, mise en mode LOCAL par defaut: {}", e);
                Connecteur::LOCAL
            });
        Self { 
            connecteur: mode_connecteur, 
            reflexion: Default::default(), 
            list_reflexions: Default::default(), 
            edit: Default::default(), 
            edit_reflexion: Default::default() 
        }
    }
}


impl Fenetre for SectionNote {
    fn name(&self) -> &'static str {
        "Reflexions"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        egui::Window::new(self.name())
        .open(is_open)
        .scroll(true)
        .show(ctx, |ui| {
            section_notes(self, ui)
        });
        Ok(())
    }
}