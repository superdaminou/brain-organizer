use std::fmt::Debug;
use egui_graphs::Graph;
use petgraph::stable_graph::StableGraph;

use crate::database::CRUD;
use crate::application_error::ApplicationError;
use crate::graph::my_graph::Graph as MyGraph;
use crate::gui::composant::EditFileable;
use crate::gui::composant::EditText;
use crate::gui::structs::Fenetre;

use super::gui::graph_window;
use super::gui_graph::GuiNode;
use ilmen_dot_parser::Node as MyNode;

use anyhow::{Context, Result};

pub struct FenetreGraph {
    pub graph: MyGraph,
    pub loaded_graph: Graph<GuiNode, String>, 
    pub creating_graph: String,
    pub graphs: Vec<MyGraph>,
    pub selected_node: Option<MyNode>,
    pub edit: EditText,
    pub edit_graph: EditFileable<MyGraph>
}

impl Debug for FenetreGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FenetreGraph").finish_non_exhaustive()
    }
}

impl Default for FenetreGraph {
    fn default() -> Self {
        Self {
            graph: MyGraph::default(),
            creating_graph: String::default(),
            graphs: MyGraph::get_all().unwrap_or_default(),
            loaded_graph: Graph::new(StableGraph::new()),
            selected_node: None,
            edit_graph: EditFileable::default(),
            edit: EditText::default()
        }
    } 
}

impl Fenetre for FenetreGraph {
    fn name(&self) -> &'static str {
        "My Graphs"
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
