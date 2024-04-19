use log::info;
use crate::application::error::ApplicationError;
use super::{reference::reference_gui::section_references, reflexion::gui::section_reflexions, structs::TemplateApp};

pub fn running_gui() -> Result<(), ApplicationError>{
    // OPEN GUI
    info!("Getting gui context");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    
    info!("Starting eframe");
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    ).map_err(ApplicationError::from)
}

pub fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
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

pub fn central_panel(template: &mut TemplateApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {        
        if section_references(&mut template.section_reference, ui).is_err() {
            template.error.visible = true;
        }
        ui.separator();

        if section_reflexions(&mut template.section_reflexion, ui).is_err(){
            template.error.visible = true;
        }
        ui.separator();
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });
}

pub fn error_panel(template: &mut TemplateApp, ctx: &egui::Context) {
    egui::Window::new("Shit, an error")
                .open(&mut template.error.visible)
                .show(ctx, |ui| {
                    ui.label(template.error.msg.clone());
                });
}

