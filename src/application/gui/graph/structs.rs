use std::collections::HashMap;

use egui::emath::Numeric;
use egui::Pos2;
use egui_graphs::Node as EguiNode;
use egui_graphs::Graph;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoEdgeReferences as _;
use rand::Rng;
use uuid::Uuid;

use crate::application::graph::structs::my_node::NodeType;
use crate::application::graph::structs::NodeGraph;
use crate::application::{error::ApplicationError, graph::structs::{edge_type::Type, my_edge::MyEdge, my_node::MyNode}, gui::structs::Fenetre};

use super::gui::graph_window;

use anyhow::{Context, Result};

pub struct FenetreGraph {
    pub graph: Graph<GuiNode, MyEdge>,
    pub selected_node: Option<GuiNode>,
    pub create_node_in: MyNode,
    pub create_node_out: MyNode,
    pub create_edge_type: Type,
    pub search: String
}

#[derive(Clone)]
#[derive(Default)]
pub struct GuiNode {
    pub node: MyNode,
    pub location: Pos2
}

impl From<&EguiNode<GuiNode, MyEdge>> for MyNode {
    fn from(value: &EguiNode<GuiNode, MyEdge>) -> Self {
        MyNode {
            id: value.payload().node.id,
            node_type: NodeType::Autre,
            identifier: value.label()
        }
    }
}


impl From<MyNode> for GuiNode {
    fn from(value: MyNode) -> Self {
        let location: Pos2 = (&value).into();
        Self { node: value, location}
    }
}


impl From<&MyNode> for GuiNode {
    fn from(value: &MyNode) -> Self {
        Self { node: value.clone(), location: value.into() }
    }
}

impl From<&MyNode> for Pos2 {
    fn from(_: &MyNode) -> Self {
        let mut rng = rand::thread_rng();
        
        let rand = rng.gen_range(0..100);

        let x = f32::from_f64(rand.to_f64().cos() * 50.0);
        let y = f32::from_f64(rand.to_f64().sin() * 50.0);
        Pos2::new(x, y)
    }
}
impl From<GuiNode> for MyNode {
    fn from(val: GuiNode) -> Self {
        val.node
    }
}




impl Default for FenetreGraph {
    fn default() -> Self {
        Self {
            graph: Graph::from(&StableGraph::<GuiNode, MyEdge>::new()),
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

pub struct GuiGraph(pub StableGraph<GuiNode, MyEdge>);

impl From<NodeGraph> for GuiGraph {
    fn from(value: NodeGraph) -> Self {
        let mut graph = StableGraph::<GuiNode, MyEdge>::new();
    
        let mut map_vertex_indice: HashMap<Uuid, petgraph::prelude::NodeIndex> = HashMap::default();

        value.0.node_weights().collect::<Vec<&MyNode>>()
            .iter()
            .for_each(|n| {
                let index = graph.add_node(GuiNode::from(*n));
                map_vertex_indice.insert(n.id, index);
            });

        value.0.edge_references().for_each(|edge| {
            graph.add_edge(edge.source(), edge.target(), edge.weight().clone());
            let source_node_pos = graph.node_weight(edge.source()).expect("Source node").location;
            let mut rng = rand::thread_rng();
                    
            let rand = rng.gen_range(0..100);

            let new_x_pos = f32::from_f64(source_node_pos.x.to_f64() + (rand.to_f64().cos() * value.0.node_count().to_f64().max(5.0) * 10.));
            let new_y_pos = f32::from_f64(source_node_pos.y.to_f64() + (rand.to_f64().sin() * value.0.node_count().to_f64().max(5.0) * 10.));

            graph.node_weight_mut(edge.target()).expect("Should have a node").location = Pos2::new(new_x_pos, new_y_pos);
        });
        GuiGraph(graph)
    }
}