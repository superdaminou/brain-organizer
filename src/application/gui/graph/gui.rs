use egui::Ui;
use egui_graphs::{ default_edge_transform, default_node_transform, to_graph_custom, DefaultEdgeShape, DefaultNodeShape, Edge, Graph, GraphView, Node as ENode, SettingsInteraction, SettingsNavigation, SettingsStyle};
use log::info;
use petgraph::{csr::DefaultIx, graph::{EdgeIndex, NodeIndex}, Directed};
use strum::IntoEnumIterator;


use crate::application::graph::{lib::{get_graph, save_node, get_node}, structs::{MyEdge, MyNode, Type}};

use super::structs::FenetreGraph;
use anyhow::{Ok, Result};



pub fn show_graph(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<()>{

    create_relation(fenetre, ui)?;
    find_node(fenetre, ui)?;
    selected_node(fenetre, ui);
    if ui.button("Load graph").clicked() {
        fenetre.graph =  actualize_graph(ui)?;
    }
    create_graph(ui, &mut fenetre.graph);
    Ok(())
}

fn find_node(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<()>{
    ui.horizontal(|ui| {
        ui.label("Nom du noeud Entrant");
        ui.text_edit_singleline(&mut fenetre.search);
        if ui.button("Getting node").clicked() {
            let node =get_node(&fenetre.search)?;
            fenetre.selected_node = Some(node);
            let gui_node = fenetre.graph.nodes_iter()
                .find(|node| node.1.payload().name == fenetre.search);
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
        Ok(())
    }).inner?;
    Ok(())
}

fn selected_node(fenetre: &mut FenetreGraph, ui:&mut Ui) {
    if !fenetre.graph.selected_nodes().is_empty() {
        fenetre.selected_node = fenetre.graph.selected_nodes().first()
            .and_then(|node_index| fenetre.graph.node(*node_index))
            .map(MyNode::from)
            .or(Some(MyNode::default()));
    } else {
        fenetre.selected_node = None
    }

    ui.label(format!("This is the selected none: {}", fenetre.selected_node.clone().unwrap_or_default().name));
}

fn create_relation(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<()> {
    ui.horizontal(|ui| {
        ui.label("Nom du noeud Sortant");
        ui.text_edit_singleline(&mut fenetre.create_node_out_name);

        egui::ComboBox::from_id_source("Tags").selected_text(fenetre.create_edge_type.to_string())
            .show_ui(ui, |ui| {
                Type::iter().for_each(|t| {
                    let type_label = ui.selectable_label(fenetre.create_edge_type == t, t.to_string());
                    if type_label.clicked() {
                        fenetre.create_edge_type = t;
                    }
                });
            }
        );

        ui.label("Nom du noeud Entrant");
        ui.text_edit_singleline(&mut fenetre.create_node_in_name);
        
        if ui.button("Add Node").clicked() {
            save_node(&fenetre.create_node_in_name, &fenetre.create_node_out_name, &fenetre.create_edge_type)?;
            fenetre.graph= actualize_graph(ui)?;
        }
        Ok(())
    }).inner
}

fn create_graph(ui:&mut Ui, graph: &mut Graph<MyNode, MyEdge>) {
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
        .with_edge_selection_enabled(true))
    .with_styles(&SettingsStyle::new().with_labels_always(true)));
}


fn actualize_graph(ui: &mut Ui) -> Result<egui_graphs::Graph<MyNode, MyEdge>> {
    GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
    Ok(to_graph_custom::<>(
            &get_graph()?, 
            node_transform, 
            edge_transform))
}
pub fn node_transform(
    idx: NodeIndex<u32>,
    payload: &MyNode,
) -> ENode<MyNode, MyEdge> {
    default_node_transform::<MyNode,MyEdge, Directed, u32,DefaultNodeShape>(idx , payload)
        .with_label(payload.name.clone())  
}

pub fn edge_transform(
    idx: EdgeIndex<u32>,
    payload: &MyEdge,
    order: usize,
) -> Edge<MyNode, MyEdge> {
    default_edge_transform::<MyNode,MyEdge,Directed,u32, DefaultNodeShape, DefaultEdgeShape>(idx , payload, order)
        .with_label(payload.edge_type.to_string())
}
