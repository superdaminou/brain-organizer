use crate::{error::ApplicationError, gui::structs::Fenetre, tag::Tag};

use super::panel_gui::panel_tags;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Default)]
pub struct PanelTags {
    pub tag: Tag,
    pub list_tags: Vec<Tag>,
}



impl Fenetre for PanelTags {
    fn name(&self) -> &'static str {
        "References"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        let visible =  egui::Window::new(self.name())
        .open(is_open)
        .scroll2(true)
        .show(ctx, |ui| {
            panel_tags(self, ui)
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