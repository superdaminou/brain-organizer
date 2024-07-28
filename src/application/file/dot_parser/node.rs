use crate::application::error::ApplicationError;

use super::attribute::{extract_attributes, Attribut};


#[derive(PartialEq, Eq, Debug)]
pub struct Node(pub String,pub Vec<Attribut>);

impl TryFrom<&String> for Node {
    type Error = ApplicationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {

        let split = value.split_once(" ").unwrap_or((value, ""));
        let mut attr = vec![];
        if !split.1.is_empty() {
            attr = extract_attributes(split.1)?
        }
        
        Ok(Self(split.0.to_string(), attr))
    }
}
