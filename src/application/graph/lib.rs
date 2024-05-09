
use std::collections::HashMap;

use indradb::{Identifier, Json, QueryExt, SpecificVertexQuery, VertexWithPropertyPresenceQuery, VertexWithPropertyValueQuery};
use log::info;
use petgraph::{ stable_graph::StableGraph, prelude::NodeIndex};
use uuid::Uuid;
use anyhow::{Context, Result};

use super::structs::{MyEdge, MyNode, Type};

pub const PERSONNE_IDENTIFIER : &str= "Personne";

pub fn get_graph() -> Result<StableGraph<MyNode, MyEdge>>{
    info!("Getting ell");
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    // Query for the edge
    let query_vertex: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery.properties()?)?;
    // Convenience function to extract out the edges from the query results
    let vertexs = indradb::util::extract_vertex_properties(query_vertex).expect("Expect some edges");
    
    let query_edge: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery)?;
    // Convenience function to extract out the edges from the query results
    let edges: Vec<indradb::Edge> = indradb::util::extract_edges(query_edge).unwrap();

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
        let default : u32 = 0;
        let binding = NodeIndex::from(default);
        let node_out= map_vertex_indice.get(&edge.outbound_id).unwrap_or(&binding);
        let node_in= map_vertex_indice.get(&edge.inbound_id).unwrap_or(&binding);

        let edge = MyEdge::try_from(edge).expect("Getting edge");
        graph.add_edge(*node_out,*node_in,  edge);
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(graph)
}



pub fn save_node(node_in: &String, node_out: &String, edge: &Type) -> Result<()> {
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    let identifier= indradb::Identifier::new(PERSONNE_IDENTIFIER)?;

    let q = VertexWithPropertyValueQuery::new(identifier, Json::new(serde_json::Value::String(node_in.clone())));
    let first_binding = indradb::util::extract_vertices(db.get(q)?)
        .unwrap_or_default();
    let binding = indradb::Vertex::new(identifier);
    let vertex_in   = first_binding
        .first()
        .unwrap_or(&binding);

    if db.create_vertex(vertex_in)? {
        info!("Successfuly created: {}", node_in);
        db.set_properties(SpecificVertexQuery::single(vertex_in.id), identifier, &indradb::Json::new(serde_json::Value::String(node_in.clone())))?;
    } else {
        info!("Already exist: {}", node_in);
    }

    let query_out = VertexWithPropertyValueQuery::new(identifier, Json::new(serde_json::Value::String(node_out.clone())));
    let first_binding = indradb::util::extract_vertices(db.get(query_out)?)
        .unwrap_or_default();
    let binding = indradb::Vertex::new(identifier);
    let vertex_out   = first_binding
        .first()
        .unwrap_or(&binding);

    if db.create_vertex(vertex_out)? {
        info!("Successfuly created: {}", node_out);
        db.set_properties(SpecificVertexQuery::single(vertex_out.id), identifier, &indradb::Json::new(serde_json::Value::String(node_out.clone())))?;
    } else {
        info!("Already exist: {}", node_out);
    }

    let edge = indradb::Edge::new(vertex_out.id, indradb::Identifier::new(edge.identifier())?, vertex_in.id);
    db.create_edge(&edge)?;

    Ok(())
}

pub fn get_node(name: &String) -> Result<MyNode> {
    info!("Getting node: {}", name);
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    let q = VertexWithPropertyPresenceQuery::new(Identifier::new(PERSONNE_IDENTIFIER)?);

    let result= db.get(q.properties()?)?;

    // Convenience function to extract out the edges from the query results
    return indradb::util::extract_vertex_properties(result).context("Extracting vertex properties")?
        .first()
        .map(MyNode::from)
        .context("Not found");
}


fn import_from_file() -> Result<StableGraph<MyNode, MyEdge>> {

    let a=  StableGraph::new();

    return  anyhow::Ok(a);
}