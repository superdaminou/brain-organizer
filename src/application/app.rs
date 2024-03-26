use rusqlite::Connection;

use super::reference::Reference;
use super::{reference, database, evenement};
use super::evenement::Evenement;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    evenement: Evenement,
    section_reference: SectionReference,
    evnements: Vec<Evenement>,

    #[serde(skip)] 
    connection: Connection
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReference {
    pub reference: Reference,
    pub list_references: Vec<Reference>,
}

impl SectionReference {
    pub fn new() -> Self {
        let connection = database::opening_database().unwrap();
        Self {
            reference: Reference { id: None, titre: "titre".to_string(), url: "String".to_string(), categorie: "Nope".to_string() },
            list_references: reference::get_all(&connection).unwrap_or(vec![])
        }
    } 
}

impl Default for TemplateApp {
    fn default() -> Self {
        let connection = database::opening_database().unwrap();
        let contenu = reference::get_all(&connection).unwrap_or(vec![]);
        let evenements = evenement::get_all(&connection).unwrap_or(vec![]);

        Self {
            // Example stuff:
            evenement: Evenement { id: None, titre: "titre".to_string(), niveau: "val".to_string() },
            section_reference: SectionReference::new(),
            connection: connection,
            evnements: evenements
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /* 
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update<'a>(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        central_panel(self, ctx); 
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn central_panel<'a>(template: &mut TemplateApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        evenement::section_evenements(&mut &template.evnements , &mut template.evenement, ui, &template.connection);
        ui.separator();
        
        reference::section_references(&mut template.section_reference, ui, &template.connection);
        ui.separator();
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });
}