
use egui_graphs::Graph;
use petgraph::stable_graph::StableGraph;

use crate::application::{graph::structs::{MyEdge, MyNode, Type}, gui::structs::Fenetre};

use super::gui::show_graph;


pub struct FenetreGraph {
    pub graph: Graph<MyNode, MyEdge>,
    pub selected_node: Option<MyNode>,
    pub create_node_in_name: String,
    pub create_node_out_name: String,
    pub create_edge_type: Type
}


impl Default for FenetreGraph {
    fn default() -> Self {
        Self {
            graph: Graph::from(&StableGraph::<MyNode, MyEdge>::new()),
            selected_node: Option::default(),
            create_node_in_name: String::default(),
            create_node_out_name: String::default(),
            create_edge_type: Type::DEFINIE
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