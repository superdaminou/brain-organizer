use std::f32::consts::E;

use egui::{Id, Pos2, Sense, Ui, Widget};
use egui_graphs::{add_node, default_edge_transform, default_node_transform, to_graph, to_graph_custom, DefaultEdgeShape, DefaultNodeShape, DisplayEdge, DisplayNode, Edge, Graph, GraphView, Node as ENode, NodeProps, SettingsInteraction, SettingsNavigation, SettingsStyle};
use log::{error, info};
use petgraph::{csr::{DefaultIx, IndexType}, graph::{EdgeIndex, Node, NodeIndex}, stable_graph::StableGraph, visit::NodeRef, Directed, EdgeType};
use serde::de;
use uuid::Uuid;


use crate::application::{graph::{lib::{get_graph, save_node}, structs::{MyEdge, MyNode}}, gui::structs::TemplateApp};

use super::{node_shape::MyNodeShape, structs::FenetreGraph};



pub fn show_graph(fenetre: &mut FenetreGraph, ui:&mut Ui) {
              

                    ui.horizontal(|ui| {
                        ui.label("Nom du noeud ");
                        ui.text_edit_singleline(&mut fenetre.node_name);
                        if ui.button("Add Node").clicked() {
                            match save_node(&fenetre.node_name) {
                                Ok(r) => info!("Okay: {}", r),
                                Err(e) => error!("{}",e.to_string())
                            }
                            fenetre.graph = Graph::from(&get_graph().unwrap());
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Add Edge").clicked() {
                            match save_node(&fenetre.node_name) {
                                Ok(r) => info!("Okay: {}", r),
                                Err(e) => error!("{}",e.to_string())
                            }
                            fenetre.graph = Graph::from(&get_graph().unwrap());
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("get Graph").clicked() {
                            fenetre.graph= Graph::from(&get_graph().unwrap());
                            fenetre.graph = 
                            to_graph_custom::<>(
                                &get_graph().unwrap(), 
                                node_transform::<MyNode, MyEdge, Directed, u32, DefaultNodeShape>, 
                                edge_transform::<MyNode, MyEdge,Directed, u32, DefaultNodeShape, DefaultEdgeShape>);
                            //fenetre.graph.add_node_with_label(MyNode::default(), "Wallalal".to_string());
                        }
    
                        if (ui.button("Reset ui")).clicked() {
                            GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
                        }
                    });


                    if !fenetre.graph.selected_nodes().is_empty() {
                        fenetre.node = fenetre.graph.selected_nodes().first()
                            .and_then(|node_index| fenetre.graph.node(*node_index))
                            .map(MyNode::from)
                            .or(Some(MyNode::default()));
                    }

                    if fenetre.graph.selected_nodes().is_empty() {
                        fenetre.node = None
                    }

                    ui.label(format!("This is the selected none: {}", fenetre.node.clone().unwrap_or_default().name));


                    create_graph(ui, &mut fenetre.graph)
}

fn create_graph(ui:&mut Ui, graph: &mut Graph<MyNode, MyEdge>) -> () {
    ui.add(&mut GraphView::<
        _,
        _,
        _,
        _,
        DefaultNodeShape,
        DefaultEdgeShape,
    >::new(graph)
    .with_navigations(
        &SettingsNavigation::new()
        .with_fit_to_screen_enabled(true))
    .with_interactions(
        &SettingsInteraction::new()
        .with_node_clicking_enabled(true)
        .with_node_selection_enabled(true)
        .with_edge_clicking_enabled(true)
        .with_edge_selection_enabled(true)));
}


pub fn node_transform<
    N: Clone,
    E: Clone,
    Ty: EdgeType,
    Ix: IndexType,
    D: DisplayNode<N, E, Ty, Ix>,
>(
    idx: NodeIndex<u32>,
    payload: &MyNode,
) -> ENode<MyNode, MyEdge> {
    return default_node_transform::<MyNode,MyEdge, Directed, u32,DefaultNodeShape>(idx , &payload)
        .with_label(payload.name.clone());
    
}

pub fn edge_transform<
    N: Clone,
    E: Clone,
    Ty: EdgeType,
    Ix: IndexType,
    Dn: DisplayNode<N, E, Ty, Ix>,
    D: DisplayEdge<N, E, Ty, Ix, Dn>,
>(
    idx: EdgeIndex<u32>,
    payload: &MyEdge,
    order: usize,
) -> Edge<MyNode, MyEdge> {
    return default_edge_transform::<MyNode,MyEdge,Directed,u32, DefaultNodeShape, DefaultEdgeShape>(idx , &payload, order)
        .with_label(payload.edge_type.to_string());
    
}
