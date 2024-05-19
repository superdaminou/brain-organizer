use std::collections::HashMap;

use indradb::{BulkInsertItem, Database, Identifier, Json, QueryExt, RocksdbDatastore, SpecificVertexQuery, VertexWithPropertyValueQuery};
use log::{debug, info};
use petgraph::stable_graph::StableGraph;
use serde_json::json;
use uuid::Uuid;
use anyhow::{Context, Result};

use crate::application::error::ApplicationError;

use super::structs::{my_edge::MyEdge, my_node::MyNode, relation::Relations, NodeGraph};


pub const NODE_TYPE : &str= "NodeType";
pub const IDENTIFIER : &str= "Identifier";
pub const DATABASE_NAME : &str = "graph.h";

pub struct  Graph {}
impl GraphDatabase for Graph {

fn get_graph() -> Result<NodeGraph, ApplicationError>{
    debug!("Getting all");
    let db =  get_database()?;
    
    let vertexs =db.get(indradb::AllVertexQuery.properties().context("With Properties")?)
        .map(indradb::util::extract_vertex_properties)?
        .expect("Querying all Nodes");
    
    let edges = db.get(indradb::AllEdgeQuery)
        .map(indradb::util::extract_edges)?
        .expect("Querying all Edges");

    let mut graph = StableGraph::<MyNode, MyEdge>::new();

    let mut map_vertex_indice: HashMap<Uuid, petgraph::prelude::NodeIndex>= 
        vertexs.iter()
        .map(|vertex| (vertex.vertex.id, petgraph::prelude::NodeIndex::default()))
        .collect();

    vertexs.iter().for_each(|node| {
        let indice = graph.add_node(MyNode::from(node));
        map_vertex_indice.insert(node.vertex.id, indice);
    });

    edges.iter().try_for_each(|edge| {
        let node_out= map_vertex_indice.get(&edge.outbound_id).context("Getting node out")?;
        let node_in= map_vertex_indice.get(&edge.inbound_id).context("Getting node in")?;

        let edge = MyEdge::try_from(edge).expect("Trying from indra edge");
        graph.add_edge(*node_out,*node_in,  edge);
        anyhow::Ok::<()>(())
    })?;

    Ok(NodeGraph(graph))
}

fn save_nodes(nodes: Vec<MyNode>) -> Result<(), ApplicationError> {
    let db = get_database()?;
    let identifier= indradb::Identifier::new(IDENTIFIER).context("Creating identifier")?;
    
    let vertexs = nodes.iter()
        .map(|node|(node, indradb::Vertex::new(identifier)))
        .collect::<Vec<_>>();

    info!("Bulk insert {} nodes", vertexs.len());
    db.bulk_insert(vertexs.iter().map(|a|BulkInsertItem::Vertex(a.1.clone())).collect())?;
    db.bulk_insert(vertexs.iter()
        .map(|node: &(&MyNode, indradb::Vertex)|BulkInsertItem::VertexProperty(node.1.id, identifier, indradb::Json::new(json!(node.0.identifier.clone())))).collect())?;
    Ok(())
}


fn save_relations(relations: Vec<Relations>) -> Result<(), ApplicationError> {
    let db = get_database()?;
    
    let mut bulk_edges = vec![];
    relations.iter()
        .try_for_each(|relation|{
            let id = indradb::Identifier::new(relation.edge.edge_type.identifier())?;
            bulk_edges.push(BulkInsertItem::Edge(indradb::Edge::new(relation.node_out.id, id ,relation.node_in.id)));
            anyhow::Ok(())
        })?;
    info!("Bulk insert {} relation", bulk_edges.len());
    db.bulk_insert(bulk_edges)?;
    
    Ok(())
}


fn save_relation(relation: Relations) -> Result<(), ApplicationError> {
    let db  = get_database()?;

    let node_in = get_or_create_vertex(relation.node_in, &db)?;
    let node_out = get_or_create_vertex(relation.node_out, &db)?;

    let edge = indradb::Edge::new(node_out.id, indradb::Identifier::new(relation.edge.edge_type.identifier()).context("Creating identifier")?, node_in.id);
    db.create_edge(&edge)?;

    Ok(())
}

fn get_node(name: &str) -> Result<MyNode, ApplicationError> {
    info!("Getting node with identifier: {}", name);
    let db = get_database()?;
    let q = VertexWithPropertyValueQuery::new(Identifier::new(IDENTIFIER).context("Creating identifier")?, Json::new(json!(name)));
    
    let result= db.get(q.properties().context("Getting properties")?)?;

    // Convenience function to extract out the edges from the query results
    return indradb::util::extract_vertex_properties(result)
        .context("Extracting vertex properties")?
        .first()
        .map(MyNode::from)
        .ok_or(ApplicationError::NotFoundError(name.to_string()));
}

fn get_node_with_relation(node: &MyNode) -> Result<NodeGraph, ApplicationError>{
    info!("Getting {}", node.identifier);
    let db =  get_database()?;
    let query = SpecificVertexQuery::single(node.id);

    let binding = db.get(query.clone().properties().context("Getting node properties")?)
        .map(|output|indradb::util::extract_vertex_properties(output.clone()).unwrap_or_default())?;
    let result= binding
        .first()
        .context("Should have found a node")?;

    let outbounds_edges = db.get(query.clone().include().outbound().context("Getting outbound")?)
        .map(|output|indradb::util::extract_edges(output.clone()).unwrap_or_default())?;

    let inbound_edges = db.get(query.clone().include().inbound().context("Getting inbound")?)
    .map(|output|indradb::util::extract_edges(output.clone()).unwrap_or_default())?;

    let mut graph = StableGraph::<MyNode, MyEdge>::new();    

    let node = graph.add_node(MyNode::from(result));

    outbounds_edges.iter().try_for_each(|edge| {
        let binding = db.get(SpecificVertexQuery::single(edge.inbound_id).clone().properties()?)
            .map(|output|indradb::util::extract_vertex_properties(output.clone()).unwrap_or_default())?;
        let inbound = binding.first()
            .expect("Should have found a node");
        
        let inbound_node = graph.add_node(MyNode::from(inbound));
        graph.add_edge(node,inbound_node,MyEdge::try_from(edge.t.to_string()).unwrap());
        anyhow::Ok(())
    })?;

    inbound_edges.iter().try_for_each(|edge| {
        let binding = db.get(SpecificVertexQuery::single(edge.outbound_id).clone().properties()?)
            .map(|output|indradb::util::extract_vertex_properties(output.clone()).unwrap_or_default())?;
        let outbound = binding.first()
            .expect("Should have found a node");
        
        let outbound_node = graph.add_node(MyNode::from(outbound));
        graph.add_edge(outbound_node,node,MyEdge::try_from(edge.t.to_string()).unwrap());
        anyhow::Ok(())
    })?;
    
    Ok(NodeGraph(graph))
}

}

fn identifier(key: &str) -> Result<indradb::Identifier> {
    indradb::Identifier::new(key).context("Creating identifier")
}

fn get_or_create_vertex(node: MyNode, db: &Database<RocksdbDatastore>) -> Result<indradb::Vertex, ApplicationError>{
    let key = identifier(IDENTIFIER)?;
    return match db.get(VertexWithPropertyValueQuery::new(key, Json::new(json!(node.identifier.clone()))))
        .map(indradb::util::extract_vertices)?
        .unwrap_or_default()
        .first() {
            Some(v) => Ok(v.to_owned()),
            None => create_vertex(db, node).map_err(ApplicationError::Other)
        };
}

fn create_vertex(db: &indradb::Database<RocksdbDatastore>,node: MyNode) -> Result<indradb::Vertex> {
    let key = identifier(IDENTIFIER)?;
    let node_type= identifier(NODE_TYPE)?;
    let vertex = indradb::Vertex::new(key);
    db.create_vertex(&vertex)?;
    db.set_properties(SpecificVertexQuery::single(vertex.id), key, &indradb::Json::new(json!(node.identifier)))?;
    db.set_properties(SpecificVertexQuery::single(vertex.id), node_type, &indradb::Json::new(json!(node.node_type.to_string())))?;
    Ok(vertex)
}

fn get_database() -> Result<Database<RocksdbDatastore>, ApplicationError> {
    debug!("Opening graph database: {}", DATABASE_NAME);
    indradb::RocksdbDatastore::new_db(DATABASE_NAME).map_err(ApplicationError::Indra)
}


pub trait GraphDatabase {
    fn get_graph() -> Result<NodeGraph, ApplicationError>;
    fn save_nodes(nodes: Vec<MyNode>) -> Result<(), ApplicationError>;
    fn save_relations(relations: Vec<Relations>) -> Result<(), ApplicationError>;
    fn save_relation(relation: Relations) -> Result<(), ApplicationError>;
    fn get_node(name: &str) -> Result<MyNode, ApplicationError>;
    fn get_node_with_relation(node: &MyNode) -> Result<NodeGraph, ApplicationError>;
}
 