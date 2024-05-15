use egui::Ui;
use egui_graphs::{ default_edge_transform, default_node_transform, to_graph_custom, DefaultEdgeShape, DefaultNodeShape, Edge, Graph, GraphView, Node as ENode, SettingsInteraction, SettingsNavigation, SettingsStyle};
use log::info;
use petgraph::{csr::DefaultIx, graph::{EdgeIndex, NodeIndex}, stable_graph::StableGraph, Directed};
use strum::IntoEnumIterator;
use uuid::Uuid;


use crate::application::{error::ApplicationError, graph::{lib::{get_graph, get_node, get_node_with_relation, save_relation}, structs::{edge_type::Type, my_edge::MyEdge, my_node::{MyNode, NodeType}, relation::Relations}}};

use super::structs::FenetreGraph;
use anyhow::Result;



pub fn graph_window(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{

    create_relation(fenetre, ui)?;
    find_node(fenetre, ui)?;
    selected_node(fenetre, ui)?;
    if ui.button("Load graph").clicked() {
        fenetre.graph = reset_graph(ui)?;
    }
    show_graph(ui, &mut fenetre.graph);
    Ok(())
}

fn find_node(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{
    ui.horizontal(|ui| {
        ui.label("Nom du noeud");
        ui.text_edit_singleline(&mut fenetre.search);
        if ui.button("Getting node").clicked() {
            let node =get_node(&fenetre.search)?;
            fenetre.selected_node = Some(node);
            let gui_node = fenetre.graph.nodes_iter()
                .find(|node| node.1.payload().identifier == fenetre.search);
            match gui_node {
                Some(node) => {
                    let index  = node.0;
                    fenetre.graph.set_selected_nodes(vec![index]);
                    fenetre.graph
                        .node_mut(index)
                        .unwrap()
                        .set_selected(true);
                },
                None => info!("Node not found in gui graph: {}", fenetre.search)
            }
        };
        Ok::<(), ApplicationError>(())
    }).inner?;
    Ok(())
}

fn selected_node(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{
    if !fenetre.graph.selected_nodes().is_empty() {
         let node = fenetre.graph.selected_nodes().first()
            .and_then(|node_index| fenetre.graph.node(*node_index))
            .map(MyNode::from);
        fenetre.selected_node = node.clone();
        if node.is_some() {
            fenetre.graph = to_egui_graph(get_node_with_relation(&node.unwrap())?)?;
        }
    } else {
        fenetre.selected_node = None
    }

    ui.label(format!("This is the selected none: {}", fenetre.selected_node.clone().unwrap_or_default().identifier));
    Ok(())
}

fn new_node(node: &mut MyNode, ui:&mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Identifiant");
        ui.text_edit_singleline(&mut node.identifier);
        egui::ComboBox::from_id_source(Uuid::new_v4()).selected_text(node.node_type.to_string())
            .show_ui(ui, |ui| {
                NodeType::iter().for_each(|t| {
                    let type_label = ui.selectable_label(node.node_type == t, t.to_string());
                    if type_label.clicked() {
                        node.node_type = t;
                    }
                });
            }
        );
    });
}

fn create_relation(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError> {
    ui.horizontal(|ui| {
        new_node(&mut fenetre.create_node_out, ui);
        egui::ComboBox::from_id_source("Tags").selected_text(fenetre.create_edge_type.to_string())
            .show_ui(ui, |ui| {
                Type::iter().for_each(|t| {
                    let type_label = ui.selectable_label(fenetre.create_edge_type == t, t.to_string());
                    if type_label.clicked() {
                        fenetre.create_edge_type= t;
                    }
                });
            }
        );
        new_node(&mut fenetre.create_node_in, ui);
        
        if ui.button("Add Node").clicked() {
            let edge =  MyEdge::from(fenetre.create_edge_type.clone());
            save_relation(Relations{node_out: fenetre.create_node_out.clone() ,edge, node_in: fenetre.create_node_in.clone()})?;
            fenetre.graph= reset_graph(ui)?;
        }
        Ok(())
    }).inner
}

fn show_graph(ui:&mut Ui, graph: &mut Graph<MyNode, MyEdge>) {
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
        .with_edge_selection_enabled(true)
        .with_dragging_enabled(true))
    .with_styles(&SettingsStyle::new().with_labels_always(true)));
}


fn reset_graph(ui: &mut Ui) -> Result<egui_graphs::Graph<MyNode, MyEdge>, ApplicationError> {
    GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
    to_egui_graph(get_graph()?)
}

fn to_egui_graph(graph: StableGraph<MyNode, MyEdge> ) -> Result<egui_graphs::Graph<MyNode, MyEdge>, ApplicationError> {
    Ok(to_graph_custom::<>(
            &graph, 
            node_transform, 
            edge_transform))
}

pub fn node_transform(
    idx: NodeIndex<u32>,
    payload: &MyNode,
) -> ENode<MyNode, MyEdge> {
    default_node_transform::<MyNode,MyEdge, Directed, u32,DefaultNodeShape>(idx , payload)
        .with_label(payload.identifier.clone())  
}

pub fn edge_transform(
    idx: EdgeIndex<u32>,
    payload: &MyEdge,
    order: usize,
) -> Edge<MyNode, MyEdge> {
    default_edge_transform::<MyNode,MyEdge,Directed,u32, DefaultNodeShape, DefaultEdgeShape>(idx , payload, order)
        .with_label(payload.edge_type.to_string())
}
