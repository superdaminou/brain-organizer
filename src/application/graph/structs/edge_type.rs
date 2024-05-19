use core::fmt;

use indradb::Identifier;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;


use crate::application::error::ApplicationError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,  EnumIter)]
pub enum Type {
    ALieuA,
    Definie,
    Default
}

impl TryFrom<Identifier> for Type {
    type Error = ApplicationError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        let value: Result<Type, ApplicationError> = match value.to_lowercase().as_str() {
            "definie" => Ok(Type::Definie),
            "a_lieu_a" => Ok(Type::ALieuA),
            "default" => Ok(Type::Default),
            _ => Err(ApplicationError::EnumError(value.to_string()))
        };
        value
    }
}

impl Type {
    pub fn identifier(&self) -> &'static str {
        match self {
            Type::ALieuA => "a_lieu_a",
            Type::Definie => "definie",
            Type::Default => "default"
        }
    }
}



impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::ALieuA => write!(f, "A eu lieu a "),
            Type::Definie => write!(f,"Définis"),
            Type::Default => write!(f, "")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier() {
        assert_eq!(Type::ALieuA.identifier(), "a_lieu_a");
    }

    #[test]
    fn try_from_identifier() {
        assert_eq!(Type::try_from(Identifier::new("definie").unwrap()).unwrap(), Type::Definie);
    }

    #[test]
    fn try_from_unknown_identifier() {
        let result = Type::try_from(Identifier::new("unexpected").unwrap());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Could not determine enum from: unexpected" )
    }

}