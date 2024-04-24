use serde::{Deserialize, Serialize};

use crate::application::{error::ApplicationError, gui::structs::Fenetre, reference::{service::get_all, structs::{reference::Reference, tag::Tag}}};

use super::reference_gui::section_references;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReferenceGui {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub tags: Vec<Tag>
}


impl From<Reference> for ReferenceGui {
    fn from(value: Reference) -> Self {
        ReferenceGui {
            id: value.id,
            titre: value.titre,
            url: value.url,
            tags: value.categorie
        }
    }
}

impl From<ReferenceGui> for Reference {
    fn from(val: ReferenceGui) -> Self {
        Reference {
            id: val.id,
            titre: val.titre,
            url: val.url,
            categorie: val.tags,
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReference {
    pub reference: ReferenceGui,
    pub list_references: Vec<ReferenceGui>,
    pub show: bool
}


impl Default for SectionReference {
    fn default() -> Self {
        Self {
            reference: Reference::new().into(),
            list_references: get_all().unwrap_or_default().iter().map(|reference| ReferenceGui::from(reference.clone())).collect::<Vec<ReferenceGui>>(),
            show: false
        }
    }
}

impl ReferenceGui {
    pub fn new() -> Self {
        Reference::new().into()
    } 
}

impl Fenetre for SectionReference {
    fn name(&self) -> &'static str {
        "References"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        egui::Window::new(self.name())
        .open(is_open)
        .scroll2(true)
        .show(ctx, |ui| {
            section_references(self, ui)
        });
        Ok(())
    }
}