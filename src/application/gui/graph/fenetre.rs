use std::fmt::Debug;
use egui_graphs::Node as EguiNode;
use egui_graphs::Graph;
use ilmen_dot_parser::Attributs;
use petgraph::stable_graph::StableGraph;
use crate::application::database::CRUD;
use ilmen_dot_parser::Node as DotNode;

use crate::application::graph::my_graph::Graph as MyGraph;
use crate::application::gui::composant::EditFile;
use crate::application::gui::composant::EditText;
use crate::application::{error::ApplicationError, gui::structs::Fenetre};

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
    pub edit_graph: EditFile
}

impl Debug for FenetreGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FenetreGraph").finish_non_exhaustive()
    }
}


fn egui_node_to_dot_node(value: &EguiNode<DotNode, String>) -> DotNode {
    DotNode::new(&value.label(), Attributs::default())
}



impl Default for FenetreGraph {
    fn default() -> Self {
        Self {
            graph: MyGraph::default(),
            creating_graph: String::default(),
            graphs: MyGraph::get_all().unwrap_or_default(),
            loaded_graph: Graph::new(StableGraph::new()),
            selected_node: None,
            edit_graph: EditFile::default(),
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
