use std::collections::BTreeSet;

use log::{error, info};


use crate::{application_error::ApplicationError, connecteur::{self, Connecteur}};

use super::structs::TemplateApp;

pub fn running_gui(connecteur: Connecteur) -> Result<(), ApplicationError>{
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
        "brain manager",
        native_options,
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc, connecteur)))),
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
    error_panel(template, ctx);

    egui::SidePanel::left("Modules").show(ctx, |ui| {
        template.fenetres.iter().map(|f| f.name()).for_each(|f| {
            let label_fenetre = ui.selectable_label(template.fenetre_ouverte.contains(&f), f);
            if label_fenetre.clicked() {
                if template.fenetre_ouverte.contains(&f) {
                    template.fenetre_ouverte.remove(&f);
                } else {
                    template.fenetre_ouverte.insert(f);
                }
            
            }
        });
    });

    
    let (_, errors) : (Vec<_>, Vec<_>)= 
        template.fenetres.iter_mut().map(|f| {
            let mut is_open = template.fenetre_ouverte.contains(f.name());
            let window = f.show(ctx, &mut is_open);
            set_open(&mut template.fenetre_ouverte, f.name(), is_open);
            window
        })
        .partition(Result::is_ok);

    let errors = errors.into_iter().map(Result::unwrap_err).collect::<Vec<ApplicationError>>();
    if !errors.is_empty() {
        template.error.visible = true;
        errors.iter().for_each(|err| error!("{}", err.to_string()));
        template.error.msg = errors.iter().map(ApplicationError::to_string).collect::<Vec<String>>().join("\\n");
    }

    egui::CentralPanel::default().show(ctx, |ui| {        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });

    });
}

fn set_open(open: &mut BTreeSet<&'static str>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key);
        }
    } else {
        open.remove(key);
    }
}

pub fn error_panel(template: &mut TemplateApp, ctx: &egui::Context) {
    egui::Window::new("Shit, an error")
                .open(&mut template.error.visible)
                .show(ctx, |ui| {
                    ui.label(template.error.msg.clone());
                });
}
