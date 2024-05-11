
use std::collections::HashMap;

use indradb::{BulkInsertItem, Identifier, Json, QueryExt, SpecificVertexQuery, VertexWithPropertyValueQuery};
use log::info;
use petgraph::{ stable_graph::StableGraph, prelude::NodeIndex};
use serde_json::json;
use uuid::Uuid;
use anyhow::{Context, Ok, Result};

use super::structs::{MyEdge, MyNode, Relations};

pub const NODE_TYPE : &str= "NodeType";
pub const IDENTIFIER : &str= "Identifier";

pub fn get_graph() -> Result<StableGraph<MyNode, MyEdge>>{
    info!("Getting all");
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    // Query for the edge
    let query_vertex: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery.properties()?)?;
    // Convenience function to extract out the edges from the query results
    let vertexs = indradb::util::extract_vertex_properties(query_vertex).expect("Expect some edges");
    
    let query_edge: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery)?;
    // Convenience function to extract out the edges from the query results
    let edges: Vec<indradb::Edge> = indradb::util::extract_edges(query_edge).unwrap();
    info!("{} edges founds", edges.len());
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

        info!("WEEEEE");
        let edge = MyEdge::try_from(edge).expect("Getting edge");
        graph.add_edge(*node_out,*node_in,  edge);
        Ok::<()>(())
    })?;

    Ok(graph)
}

pub fn save_nodes(nodes: Vec<MyNode>) -> Result<()> {
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    let identifier= indradb::Identifier::new(IDENTIFIER)?;
    
    let vertexs = nodes.iter()
        .map(|node|(node, indradb::Vertex::new(identifier)))
        .collect::<Vec<_>>();

    info!("Bulk insert {} nodes", vertexs.len());
    db.bulk_insert(vertexs.iter().map(|a|BulkInsertItem::Vertex(a.1.clone())).collect())?;
    db.bulk_insert(vertexs.iter()
        .map(|node: &(&MyNode, indradb::Vertex)|BulkInsertItem::VertexProperty(node.1.id, identifier, indradb::Json::new(json!(node.0.identifier.clone())))).collect())?;
    Ok(())
}


pub fn save_relations(relations: Vec<Relations>) -> Result<()> {
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    
    let mut bulk_edges = vec![];
    relations.iter()
        .try_for_each(|relation|{
            let id = indradb::Identifier::new(relation.edge.edge_type.identifier())?;
            bulk_edges.push(BulkInsertItem::Edge(indradb::Edge::new(relation.node_out.id.expect("Should have an id"), id ,relation.node_in.id.expect("Shoyld have"))));
            info!("A new one");
            Ok(())
        })?;
    info!("Bulk insert {} relation", bulk_edges.len());
    db.bulk_insert(bulk_edges)?;
    
    Ok(())
}


pub fn save_relation(relation: Relations) -> Result<()> {
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    let identifier= indradb::Identifier::new(IDENTIFIER)?;

    let q = VertexWithPropertyValueQuery::new(identifier, Json::new(json!(relation.node_in.identifier.clone())));
    let first_binding = indradb::util::extract_vertices(db.get(q)?)
        .unwrap_or_default();
    let binding = indradb::Vertex::new(identifier);
    let vertex_in   = first_binding
        .first()
        .unwrap_or(&binding);

    if db.create_vertex(vertex_in)? {
        info!("Successfuly created: {}", relation.node_in.identifier);
        db.set_properties(SpecificVertexQuery::single(vertex_in.id), identifier, &indradb::Json::new(json!(relation.node_in.identifier)))?;
    } else {
        info!("Already exist: {}", relation.node_in.clone().identifier);
    }

    let query_out = VertexWithPropertyValueQuery::new(identifier, Json::new(json!(relation.node_out.identifier.clone())));
    let first_binding = indradb::util::extract_vertices(db.get(query_out)?)
        .unwrap_or_default();
    let binding = indradb::Vertex::new(identifier);
    let vertex_out   = first_binding
        .first()
        .unwrap_or(&binding);

    if db.create_vertex(vertex_out)? {
        info!("Successfuly created: {}", relation.node_out.identifier);
        db.set_properties(SpecificVertexQuery::single(vertex_out.id), identifier, &indradb::Json::new(json!(relation.node_out.identifier)))?;
    } else {
        info!("Already exist: {}", relation.node_out.identifier);
    }

    let edge = indradb::Edge::new(vertex_out.id, indradb::Identifier::new(relation.edge.edge_type.identifier())?, vertex_in.id);
    db.create_edge(&edge)?;

    Ok(())
}

pub fn get_node(name: &String) -> Result<MyNode> {
    info!("Getting node with identifier: {}", name);
    let db: indradb::Database<indradb::RocksdbDatastore> = indradb::RocksdbDatastore::new_db("rock.h")?;
    let q = VertexWithPropertyValueQuery::new(Identifier::new(IDENTIFIER)?, Json::new(json!(name)));
    
    let result= db.get(q.properties()?)?;

    // Convenience function to extract out the edges from the query results
    return indradb::util::extract_vertex_properties(result).context("Extracting vertex properties")?
        .first()
        .map(MyNode::from)
        .context("Not found");
}

