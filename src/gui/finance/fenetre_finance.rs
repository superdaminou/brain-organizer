use anyhow::Context;
use crate::{application_error::ApplicationError, gui::structs::Fenetre, finance::depense::Depense};

use super::gui::finances_gui;



#[derive(Default)]
pub struct FenetreFinance {
    pub depenses: Vec<Depense>,
    pub depense: Depense
}

impl Fenetre for FenetreFinance {
    fn name(&self) -> &'static str {
        "Finances"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> anyhow::Result<(),ApplicationError> {
        let visible = egui::Window::new(self.name())
        .open(is_open)
        .show(ctx, |ui| {
            finances_gui(self, ui)
        });

        match visible {
            Some(windows) => {
                windows.inner.context("Graph GUI Error")?
            },
            None => Ok(())
        }
    }
}

