use egui::Ui;
use egui_graphs::{ default_edge_transform, default_node_transform, to_graph_custom, DefaultEdgeShape, DefaultNodeShape, DisplayEdge, DisplayNode, Edge, Graph, GraphView, Node as ENode, NodeProps, SettingsInteraction, SettingsNavigation, SettingsStyle};

use petgraph::{csr::{DefaultIx, IndexType}, graph::{EdgeIndex, NodeIndex}, Directed, EdgeType};
use strum::IntoEnumIterator;


use crate::application::{error::ApplicationError, graph::{lib::{get_graph, save_node}, structs::{MyEdge, MyNode, Type}}};

use super::structs::FenetreGraph;



pub fn show_graph(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{

    // Ajout du noeud
    ui.horizontal(|ui| {
        ui.label("Nom du noeud Entrant");
        ui.text_edit_singleline(&mut fenetre.create_node_in_name);

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


        ui.label("Nom du noeud sortant");
        ui.text_edit_singleline(&mut fenetre.create_node_out_name);
        
        if ui.button("Add Node").clicked() {
            save_node(&fenetre.create_node_in_name, &fenetre.create_node_out_name, &fenetre.create_edge_type)?;
            fenetre.graph= actualize_graph(ui);
        }
        Ok::<(), ApplicationError>(())
    });


    ui.horizontal(|ui| {
        if ui.button("get Graph").clicked() {
            fenetre.graph= actualize_graph(ui);
        }
    });


    if !fenetre.graph.selected_nodes().is_empty() {
        fenetre.selected_node = fenetre.graph.selected_nodes().first()
            .and_then(|node_index| fenetre.graph.node(*node_index))
            .map(MyNode::from)
            .or(Some(MyNode::default()));
    } else {
        fenetre.selected_node = None
    }

    ui.label(format!("This is the selected none: {}", fenetre.selected_node.clone().unwrap_or_default().name));


    create_graph(ui, &mut fenetre.graph);
    Ok(())
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


fn actualize_graph(ui: &mut Ui) -> egui_graphs::Graph<MyNode, MyEdge> {
    GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
    return to_graph_custom::<>(
        &get_graph().unwrap(), 
        node_transform::<MyNode, MyEdge, Directed, u32, DefaultNodeShape>, 
        edge_transform::<MyNode, MyEdge,Directed, u32, DefaultNodeShape, DefaultEdgeShape>);
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
