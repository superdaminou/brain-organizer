use crate::{application_error::ApplicationError, connecteur::Connecteur, finance::depense::{Depense, REPETITION}, gui::structs::Fenetre};

use super::gui::finances_gui;



pub struct SectionFinance {
    pub depenses: Vec<Depense>,
    pub depense: Depense,
    pub connecteur: Connecteur,
    pub mode_calcul: REPETITION,
    pub calcul: f32
}

impl SectionFinance {
    pub fn new(connecteur: Connecteur) -> Self {
        Self { 
            connecteur, 
            depense: Depense::default(),
            depenses: Vec::default(),
            mode_calcul: REPETITION::MENSUEL,
            calcul: 0.0
        }
    }
}


impl Fenetre for SectionFinance {
    fn name(&self) -> &'static str {
        "Finances"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(),ApplicationError> {
        let visible = egui::Window::new(self.name())
        .open(is_open)
        .show(ctx, |ui| {
            finances_gui(self, ui)
        });

        match visible {
            Some(windows) => {
                windows.inner.ok_or_else(||ApplicationError::DefaultError("Expecting something".to_string()))?
            },
            None => Ok(())
        }
    }
}

