use super::{app::central_panel, reference::structs::SectionReference, reflexion::structs::SectionReflexion};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    pub section_reference: SectionReference,
    pub section_reflexion: SectionReflexion,
    pub error: AppError,
}

impl Default for TemplateApp {
    fn default() -> Self {
        //let g = generate_graph();
        Self {
            section_reference: SectionReference::new(),
            section_reflexion: SectionReflexion::new(),
            error: AppError::init(),
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
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /* 
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */
        //let g = generate_graph();
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


// fn generate_graph() -> StableGraph<(), ()> {
//     let mut g = StableGraph::new();

//     let a = g.add_node(());
//     let b = g.add_node(());
//     let c = g.add_node(());

//     g.add_edge(a, b, ());
//     g.add_edge(b, c, ());
//     g.add_edge(c, a, ());

//     g
// }