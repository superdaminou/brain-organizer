use std::{fs::read_to_string, usize};

use anyhow::Context;
use log::debug;

use crate::application::error::ApplicationError;

use super::{dot_graph::DotGraph, struts::GraphFamily};


pub fn import_graph(path: String) -> Result<(), ApplicationError> {
    let file = read_to_string(path)
        .with_context(|| format!("Reading file {}", "graph.dot"))?;

    let cleaned_file = file.split("\r\n")
        .map(|line| line.trim_ascii())
        .filter(|line| !line.is_empty() || line.starts_with("//"))
        .collect::<Vec<&str>>().join("\r\n");


    DotGraph::try_from(cleaned_file)?;
    return Ok(());
}

