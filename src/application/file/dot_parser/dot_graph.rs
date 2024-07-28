use egui::TextBuffer;
use log::debug;

use crate::application::error::ApplicationError;

use super::struts::{GraphFamily, GraphType};

#[derive(PartialEq)]
pub struct DotGraph {
    pub family: GraphFamily, 
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub sous_graphes: Vec<DotGraph>,
    pub attributs: Attributs,
    pub name: String
}

impl Default for DotGraph {
    fn default() -> Self {
        Self { family: GraphFamily::Graph(GraphType::Graph), nodes: Default::default(), edges: Default::default(), sous_graphes: Default::default(), attributs: Default::default(), name: Default::default() }
    }
}

type Attributs = Vec<Attribut>;
type Attribut = String;
type Node = String;
type Edge = String;

impl TryFrom<String> for DotGraph {
    type Error = ApplicationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        debug!("try dotGraph from: {}", value);
        let range = block_range(&value)?
            .ok_or(ApplicationError::DefaultError("Should habe a block".to_string()))?;
        let graph_description = value.char_range(0..range.0).split_once(" ").unwrap();
        let graph_type = GraphFamily::try_from(graph_description.0.trim())?;
        let name = match graph_type {
            GraphFamily::Graph(_) => graph_description.1.trim(),
            _ => ""
        }.to_string() ;

        let sous_graphes = extract_subgraphes(&value.char_range(range.0+1..range.1))?
            .iter()
            .map(|(start,end)|DotGraph::try_from(value.char_range(*start..*end).to_string()))
            .collect::<Result<Vec<DotGraph>, ApplicationError>>()?;
    
        Ok(DotGraph {name, family: graph_type, sous_graphes, ..Default::default() })
        }
}


fn extract_subgraphes(inside_block: &str) -> Result<Vec<(usize, usize)>, ApplicationError> {
    let mut sub_graphes_ranges = vec![];
    let mut next_block = block_range(&inside_block)?;
    let mut start=0;
    while next_block.is_some() {
        sub_graphes_ranges.push((next_block.unwrap().0+start, next_block.unwrap().1+start));
        start += next_block.unwrap().1+1 ;
        next_block = block_range(&inside_block.char_range(start..inside_block.len()))?;
    }

    return Ok(sub_graphes_ranges);
}


fn block_range(block: &str) -> Result<Option<(usize, usize)>, ApplicationError>{
    let mut stack = 0;
    let mut range : (Option<usize>, Option<usize>)= (None, None);
    let mut chars = block.chars();
    

    let mut index = 0;
    let mut next= chars.next();
    while next.is_some() {
        let char = next.unwrap();
        

        if char == '{' {
            match range.0 {
                Some(_) => stack+=1,
                None => range.0 = Some(index),
            }
        }

        if char == '}' {
            match range.0 {
                Some(_) => { 
                    if stack == 0 {
                        range = (range.0, Some(index));
                        break;
                    } else {
                        stack -= 1;
                    }
                },
                None => return Err(ApplicationError::DefaultError("Missing starting {".to_string())),
            }
        } 
        index +=1;
        next = chars.next();
    }

    return match range {
        (None,None) => Ok(None),
        (Some(_), None) => Err(ApplicationError::DefaultError("Missing ending block".to_string())),
        (None, Some(_)) => Err(ApplicationError::DefaultError("Missing starting {".to_string())),
        (Some(start), Some(end)) => Ok(Some((start, end))),
    };
}




#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_find_ending_pos_combinations() {
        let combinations :Vec<(&str, (usize,usize))> = vec![
            ("{test -> a;}", (0,11)),
            ("{{}}", (0,3)),
            ("{icitoutvabien}", (0,14)),
            ("{{{{}}}}", (0,7)),
            ("{{{{}}}}}", (0,7)),
            ("graph Test { A -> B [label=\"to B\"];B -> C [label=\"to C\"];C -> D [label=\"to D\", color=red];}", (11,90))
            ];
            
        combinations.iter().for_each(|combinaisons| assert_eq!(block_range(&combinaisons.0).unwrap(), Some(combinaisons.1)));
    }

    #[test]
    fn test_find_without_block() {
        assert_eq!(block_range("a,b").unwrap(), None)
    }

    #[test]
    fn test_find_ending_pos_combinations_ko() {
        let combinations :Vec<(&str, &str)> = vec![
            ("}test{}", "Missing starting {"),
            ("{testt", "Missing ending block"),
            ("{test{}", "Missing ending block")
            ];
            
        combinations.iter().for_each(|combinaisons| assert_eq!(block_range(&combinaisons.0).unwrap_err().to_string(), ApplicationError::DefaultError(combinaisons.1.to_string()).to_string()));

    } 


    #[test]
    fn graph_try_from() {
        let input = "graph Test {\r\nA -> B [label=\"to B\"];\r\nB -> C [label=\"to C\"];\r\nC -> D [label=\"to D\", color=red];\r\n}";

        let result = DotGraph::try_from(input.to_string()).unwrap();
        assert_eq!(result.name, "Test".to_string())
    }


    #[test]
    fn extract_subgraphes_ok() {
        let combinations :Vec<(&str,usize)> = vec![
            ("{tetsautres}", 1),
            ("another what ? {tetsautres}", 1),
            ("encore un test {tetsautres} et au { } voila du boulout", 2),
            ];
            

        combinations.iter().for_each(|combinaisons| assert_eq!(extract_subgraphes(combinaisons.0).unwrap().len(), combinaisons.1));
    } 
}