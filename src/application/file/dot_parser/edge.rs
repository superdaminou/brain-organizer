use crate::application::error::ApplicationError;

use super::attribute::{extract_attributes, Attribut};

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Edge{
    pub left_node: NodeId,
    pub right_node: NodeId,
    pub relation: String,
    pub attributs: Vec<Attribut>
}

type NodeId = String;


impl TryFrom<(&str, &str)> for Edge {
    type Error = ApplicationError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let splitted = value.0
            .split_once(value.1)
            .ok_or(ApplicationError::DefaultError("wtf".to_string()))?;

        let left_node= splitted.0.trim().to_string();
        let relation = value.1.to_string();
        let right_node = splitted.1
            .split_once("[")
            .unwrap_or((splitted.1, "")).0
            .trim()
            .to_string();
        let attributs = extract_attributes(splitted.1)?;
        

        Ok(Self{left_node, right_node, relation, attributs})
    }
}

#[test]
fn try_from_ok() {
    let combinations :Vec<(&str,Edge)> = vec![
        ("A->B", Edge{left_node: "A".to_string(), right_node: "B".to_string(), relation: "->".to_string(), attributs: vec![]}),
        (" A -> B ", Edge{left_node: "A".to_string(), right_node: "B".to_string(), relation: "->".to_string(), attributs: vec![]}),
        ("A->B[toto=tutu]", Edge{left_node: "A".to_string(), right_node: "B".to_string(), relation: "->".to_string(), attributs: vec![Attribut::try_from("toto=tutu").unwrap()]})
        ];
        

    combinations.iter().for_each(|combinaisons| assert_eq!(Edge::try_from((combinaisons.0, "->")).unwrap(), combinaisons.1));
} 