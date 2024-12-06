use std::collections::HashMap;

use egui::Ui;
use egui_graphs::{ default_edge_transform, default_node_transform, to_graph_custom, Edge, Graph, GraphView, Node as ENode, SettingsInteraction, SettingsNavigation, SettingsStyle};
use log::info;
use petgraph::{ csr::DefaultIx, graph::{EdgeIndex, NodeIndex}, prelude::StableGraph, Directed};


use ilmen_dot_parser::{Attributs, DotGraph};
use ilmen_dot_parser::Node as DotNode;
use crate::{database::CRUD, application_error::ApplicationError, gui::composant::EditText};
use crate::graph::my_graph::Graph as MyGraph;

use super::{fenetre::FenetreGraph, gui_graph::{GuiGraph, GuiNode}};
use anyhow::Result;

pub fn graph_window(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{
    new_graph(fenetre, ui)?;
    
    selected_graph(fenetre, ui)?;
    selected_node(fenetre, ui)?;
    EditText::default().show(ui, &mut fenetre.edit_graph)?;

    ui.horizontal(|ui: &mut egui::Ui| {
        egui::ComboBox::from_label("Graph")
            .selected_text(format!("{:?}", fenetre.graph.filename()))
            .show_ui(ui, |ui| {
                fenetre.graphs.iter().for_each(|g| {
                    let value = ui.selectable_value(&mut &fenetre.graph, g, g.filename());
                    if value.clicked() {
                        fenetre.graph = g.clone();
                    };
                    
                })
            }
        );
        Ok::<(), ApplicationError>(())
    }).inner?;
    
    show_graph(ui, &mut fenetre.loaded_graph);
    
    Ok(())
}

fn new_graph(section: &mut FenetreGraph, ui: &mut egui::Ui) -> Result<()> {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.heading("New Graph File: ");
        ui.label("filename");
        ui.text_edit_singleline(&mut section.creating_graph);
    
        let button = egui::Button::new("Créer");
        if ui.add(button).clicked() {
            let graph = MyGraph::from(&section.creating_graph);
            MyGraph::create(&graph)?;
            section.graph = graph;
            section.graphs = MyGraph::get_all().unwrap_or_default();
        }
        anyhow::Ok(())
    }).inner?;

    Ok(())

}

fn show_graph(ui:&mut Ui, graph: &mut Graph<GuiNode, String>) {
    ui.add(&mut GraphView::<
        _,
        _,
        _,
        _,
        _,
        _,
    >::new(graph)
    .with_navigations(
        &SettingsNavigation::new()
        .with_fit_to_screen_enabled(false)
        .with_zoom_and_pan_enabled(true))
    .with_interactions(
        &SettingsInteraction::new()
        .with_node_clicking_enabled(true)
        .with_node_selection_enabled(true)
        .with_edge_clicking_enabled(true)
        .with_edge_selection_enabled(true)
        .with_dragging_enabled(true))
    .with_styles(&SettingsStyle::new().with_labels_always(true)));
}


pub fn to_egui_graph(dot_graph: DotGraph ) -> Result<egui_graphs::Graph<GuiNode, String>, ApplicationError> {
    info!("Transforming to egui graph: {}", dot_graph.name());
    let mut graph = StableGraph::<DotNode, String>::new();
        let mut index_by_node = dot_graph
            .nodes()
            .iter()
            .map(|n| (n.identifier.clone(), graph.add_node(n.clone())))
            .collect::<HashMap<String, NodeIndex>>();
        
        let edges = dot_graph.edges();

        edges.iter()
            .try_for_each(|e| {
            
            let left = index_by_node.get(&e.node_out).copied()
                .unwrap_or_else(|| insert_and_get_index(&e.node_out, &mut graph, &mut index_by_node));
            
            let right = index_by_node.get(&e.node_in).copied()
                .unwrap_or_else(|| insert_and_get_index(&e.node_in, &mut graph, &mut index_by_node));
            
            graph.add_edge(left, right, e.relation.to_string());
            Ok::<(), ApplicationError>(())
        })?;

    Ok(to_graph_custom::<>(
            &mut GuiGraph::from(graph).0, 
            node_transform, 
            edge_transform))
}

fn insert_and_get_index(node: &String, graph:&mut StableGraph::<DotNode, String>, index_by_node:&mut HashMap<String, NodeIndex>) -> NodeIndex{
    let node_index = graph.add_node(DotNode::new(node.as_str(), Attributs::default()));
    index_by_node.insert(node.clone(), node_index);
    node_index
}

pub fn node_transform(
    idx: NodeIndex<u32>,
    payload: &GuiNode,
) -> ENode<GuiNode, String> {
    let mut node = default_node_transform::<GuiNode,String, Directed, u32,_>(idx , payload)
        .with_label(payload.0.identifier.clone());
    node.set_location(payload.1);
    node
}

pub fn edge_transform(
    idx: EdgeIndex<u32>,
    payload: &String,
    order: usize,
) -> Edge<GuiNode, String> {
    default_edge_transform::<GuiNode,String,Directed,u32, _, _>(idx , payload, order)
        .with_label(payload.clone())
}


fn selected_node(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{
    match fenetre.loaded_graph.selected_nodes().first()
    .and_then(|node_index| fenetre.loaded_graph.node(*node_index).cloned())
     {
        None => fenetre.selected_node = None,
        Some(selected_node) => {
            if !selected_node.payload().0.identifier.eq(&fenetre.selected_node.clone().map(|n|n.identifier).unwrap_or("".to_string())) {
                let selected_node_index = fenetre.loaded_graph.nodes_iter()
                    .find(|n| n.1.payload().0.identifier.eq(&selected_node.payload().0.identifier))
                    .map(|(i, _)| i)
                    .unwrap();
    
                    fenetre.loaded_graph.set_selected_nodes(vec![selected_node_index]);
                    fenetre.loaded_graph.node_mut(selected_node_index).unwrap().set_selected(true);
                fenetre.selected_node =  Some(selected_node.payload().0.clone());
            }
        }
    }



    
    let selectectd_node = fenetre.selected_node.clone().unwrap_or_default();
    ui.label(format!("Selected none: {} with attributes: {}", selectectd_node.identifier, 
        selectectd_node.attributes.attributs()
            .unwrap_or_default()
            .iter()
            .map(|attribut| attribut.0.to_string() + ": " + attribut.1)
            .collect::<Vec<String>>().join(",")));
    Ok(())
}

fn selected_graph(fenetre: &mut FenetreGraph, ui:&mut Ui) -> Result<(), ApplicationError>{
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label(format!("Current graph: {}", fenetre.graph.filename()));
        if ui.button("Editer").clicked() {
            fenetre.edit.open(&fenetre.graph, &mut fenetre.edit_graph)?;
            fenetre.edit_graph.show = true;
        }

        if ui.button("Charger Graph").clicked() {
            GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
            fenetre.graph = MyGraph::get_one(&fenetre.graph.id)?;
            fenetre.loaded_graph = to_egui_graph(fenetre.graph.load_graph()?)?;
        }
        Ok::<(), ApplicationError>(())
     });

    
    Ok(())
}