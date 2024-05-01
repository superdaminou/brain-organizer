use std::{fs::{read_to_string, File}, io::Write};

use egui::{TextEdit, Ui, Window};
use egui_graphs::Graph;
use log::info;
use petgraph::stable_graph::StableGraph;

use crate::application::{error::ApplicationError, file::construct_path, graph::structs::{MyEdge, MyNode}, gui::structs::Fenetre, reflexion::{service::get_all, structs::Reflexion}};

use super::gui::show_graph;


pub struct FenetreGraph {
    pub graph: Graph<MyNode, MyEdge>,
    pub node: Option<MyNode>,
    pub node_name: String,
}


impl Default for FenetreGraph {
    fn default() -> Self {
        Self {
            graph: Graph::from(&StableGraph::<MyNode, MyEdge>::new()),
            node: Option::default(),
            node_name: String::default()
        }
    } 
}

impl Fenetre for FenetreGraph {
    fn name(&self) -> &'static str {
        "My Graph"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), crate::application::error::ApplicationError> {
        egui::Window::new(self.name())
        .open(is_open)
        .show(ctx, |ui| {
            show_graph(self, ui)
        });
        Ok(())
    }
}