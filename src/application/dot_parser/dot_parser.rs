use std::fs::read_to_string;

use anyhow::Context;
use crate::application::error::ApplicationError;

use super::dot_graph::DotGraph;


pub fn graph_from_file(path: &str) -> Result<DotGraph, ApplicationError> {
    let file = read_to_string(path)
        .with_context(|| format!("Reading file {}", "graph.dot"))?;
    
    let cleaned_file = file.split("\r\n")
        .map(|line| line.trim_ascii())
        .filter(|line| !line.is_empty() || line.starts_with("//"))
        .collect::<Vec<&str>>()
        .join("\r\n");

    DotGraph::try_from(cleaned_file.as_str())
}

