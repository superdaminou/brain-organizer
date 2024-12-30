use std::collections::BTreeSet;

use crate::{application_error::ApplicationError, connecteur::Connecteur};

use super::{app::central_panel, finance::fenetre_finance::SectionFinance, graph::fenetre::FenetreGraph, reference::panel::PanelReference, note::section_note::SectionNote};
use eframe::egui::Context;
pub struct TemplateApp {
    pub fenetres: Vec<Box<dyn Fenetre>>,
    pub error: AppError,
    pub fenetre_ouverte: BTreeSet<&'static str>
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppError {
    pub visible: bool,
    pub msg: String
}

impl AppError {
    pub fn init() -> AppError {
        AppError {
            visible: false,
            msg: String::new()
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, connecteur: Connecteur) -> Self {
        let fenetres: Vec<Box<dyn Fenetre>> = vec![
                Box::<PanelReference>::new(PanelReference::new(connecteur)),
                Box::<SectionNote>::new(SectionNote::new(connecteur)),
                Box::<FenetreGraph>::new(FenetreGraph::new(connecteur)),
                Box::<SectionFinance>::new(SectionFinance::new(connecteur))
            ];
        Self {
            fenetres,
            error: AppError::init(),
            fenetre_ouverte: BTreeSet::new()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        central_panel(self, ctx);
    }
}


pub trait Fenetre {
    // Associated function signature; `Self` refers to the implementor type.
    
    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(),ApplicationError>;
}
