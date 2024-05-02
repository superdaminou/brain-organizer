
use std::collections::HashMap;

use indradb::{Identifier, VertexWithPropertyPresenceQuery};
use log::info;
use petgraph::{ stable_graph::StableGraph, prelude::NodeIndex};
use uuid::Uuid;
use anyhow::{Context, Result};

use super::structs::{MyEdge, MyNode, Type};

pub fn get_graph() -> Result<StableGraph<MyNode, MyEdge>>{
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    // Query for the edge
    let query_vertex: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery)?;
    // Convenience function to extract out the edges from the query results
    let vertexs: Vec<indradb::Vertex> = indradb::util::extract_vertices(query_vertex).unwrap();

    let query_edge: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery)?;
    // Convenience function to extract out the edges from the query results
    let edges: Vec<indradb::Edge> = indradb::util::extract_edges(query_edge).unwrap();

    let mut graph = StableGraph::<MyNode, MyEdge>::new();

    let mut map_vertex_indice: HashMap<Uuid, petgraph::prelude::NodeIndex>= 
        vertexs.iter()
        .map(|vertex| (vertex.id, petgraph::prelude::NodeIndex::default()))
        .collect();

    vertexs.iter().for_each(|node| {
        let indice = graph.add_node(MyNode::from(node));
        map_vertex_indice.insert(node.id, indice);
        ()
    });

    edges.iter().try_for_each(|edge| {
        let default : u32 = 0;
        let binding = NodeIndex::from(default);
        let node_out= map_vertex_indice.get(&edge.outbound_id).unwrap_or(&binding);
        let node_in= map_vertex_indice.get(&edge.inbound_id).unwrap_or(&binding);

        let edge = MyEdge::try_from(edge).expect("Getting edge");
        graph.add_edge(node_out.clone(),node_in.clone(),  edge);
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(graph)
}

pub fn save_node(node_in: &String, node_out: &String, edge: &Type) -> Result<()> {
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;

    let in_id = indradb::Identifier::new(node_in).map(indradb::Vertex::new)?;
    db.create_vertex(&in_id)?;

    let out_id = indradb::Identifier::new(node_out).map(indradb::Vertex::new)?;
    db.create_vertex(&out_id)?;

    let edge = indradb::Edge::new(out_id.id, indradb::Identifier::new(edge.identifier())?, in_id.id);
    db.create_edge(&edge)?;
    info!("All is good");
    Ok(())
}

pub fn get_node(name: &String) -> Result<MyNode> {
    info!("Getting node: {}", name);
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    let q = VertexWithPropertyPresenceQuery::new(Identifier::new(name).unwrap());

    let result= db.get(q)?;
    // Convenience function to extract out the edges from the query results
    return indradb::util::extract_vertices(result).unwrap()
        .first()
        .map(MyNode::from)
        .context("Not found");
}
