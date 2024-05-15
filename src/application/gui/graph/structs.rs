
use egui_graphs::Graph;
use petgraph::stable_graph::StableGraph;

use crate::application::{error::ApplicationError, graph::structs::{edge_type::Type, my_edge::MyEdge, my_node::MyNode}, gui::structs::Fenetre};

use super::gui::graph_window;

use anyhow::{Context, Result};
pub struct FenetreGraph {
    pub graph: Graph<MyNode, MyEdge>,
    pub selected_node: Option<MyNode>,
    pub create_node_in: MyNode,
    pub create_node_out: MyNode,
    pub create_edge_type: Type,
    pub search: String
}


impl Default for FenetreGraph {
    fn default() -> Self {
        Self {
            graph: Graph::from(&StableGraph::<MyNode, MyEdge>::new()),
            selected_node: Option::default(),
            create_node_in: MyNode::default(),
            create_node_out: MyNode::default(),
            create_edge_type: Type::Definie,
            search: String::default()
        }
    } 
}

impl Fenetre for FenetreGraph {
    fn name(&self) -> &'static str {
        "My Graph"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Result<(), ApplicationError> {
        let visible = egui::Window::new(self.name())
        .open(is_open)
        .show(ctx, |ui| {
            graph_window(self, ui)
        });

        match visible {
            Some(windows) => {
                windows.inner.context("Graph GUI Error")?
            },
            None => Ok(())
        }


    }
}

