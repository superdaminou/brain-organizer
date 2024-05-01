
use std::{collections::HashMap, f32::consts::E, fs::read_to_string};

use log::info;
use petgraph::{ stable_graph::StableGraph, prelude::NodeIndex};
use uuid::Uuid;

use crate::application::error::ApplicationError;

use super::structs::{MyEdge, MyNode, Type};


pub fn get_graph() -> Result<StableGraph<MyNode, MyEdge>, ApplicationError>{
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h").map_err(ApplicationError::from)?;
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
        info!("{}", edge.t.to_string());
        graph.add_edge(node_out.clone(),node_in.clone(),  MyEdge::try_from(edge)?);
        Ok::<(), ApplicationError>(())
    })?;

    Ok(graph)
}


pub fn get_node(name: String) -> Result<StableGraph<MyNode, MyEdge>, ApplicationError>{
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h").map_err(ApplicationError::from)?;
    // Query for the vertex
    let edges: Vec<indradb::QueryOutputValue> = db.get(indradb::SpecificVertexQuery::new(vec![Uuid::new_v4()]))?;
    // Convenience function to extract out the edges from the query results
    let e = indradb::util::extract_vertices(edges).unwrap();

    let mut g = StableGraph::<MyNode, MyEdge>::new();
    
    let damien = g.add_node(MyNode::default());
    let coprs = g.add_node(MyNode::default());

    g.add_edge(damien, coprs, MyEdge {edge_type: Type::ALieuA});


    Ok(g)
}


fn read_file() -> Result<(), ApplicationError>{
    let a = read_to_string("test.lua") 
        .map_err(ApplicationError::from)?
        .lines().collect::<Vec<&str>>();

    Ok(())
}

pub fn save_node(name: &String) -> Result<Uuid, ApplicationError> {
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h").map_err(ApplicationError::from)?;
    // Create a couple of vertices
    let node = indradb::Vertex::new(indradb::Identifier::new(name).map_err(ApplicationError::from)?);
    db.create_vertex(&node)?;
    let edge = indradb::Edge::new(node.id, indradb::Identifier::new("DEFINIE").map_err(ApplicationError::from)?, node.id);
    db.create_edge(&edge)?;
    Ok(node.id)
}


// pub fn save_edge(name: String, node_in: Uuid, node_out: Uuid) -> Result<String, ApplicationError> {
//     let db: indradb::Database<indradb::RocksdbDatastore> = indradb::Database::from("rock.h").map_err(ApplicationError::from)?;
//     // Create a couple of vertices
//     let edge = indradb::Edge::new(node_out, indradb::Identifier::new("likes").map_err(ApplicationError::from)?, node_in);
//     db.create_edge(&edge)?;
//     Ok(edge.t.to_string())
// }