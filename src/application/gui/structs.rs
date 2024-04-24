use egui_graphs::Graph;
use petgraph::stable_graph::StableGraph;

use super::{app::central_panel, reference::structs::SectionReference, reflexion::structs::SectionReflexion};

pub struct TemplateApp {
    pub section_reference: SectionReference,
    pub section_reflexion: SectionReflexion,
    pub show_reference: bool,
    pub error: AppError,
    pub g: Graph<(), ()>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let g = generate_graph();
        Self {
            section_reference: SectionReference::new(),
            section_reflexion: SectionReflexion::new(),
            error: AppError::init(),
            g: Graph::from(&g),
            show_reference: false
        }
    }
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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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


fn generate_graph() -> StableGraph<(), ()> {
    let mut g = StableGraph::new();

    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());

    g.add_edge(a, b, ());
    g.add_edge(b, c, ());
    g.add_edge(c, a, ());

    g
}
