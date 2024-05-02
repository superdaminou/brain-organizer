use std::fmt::{self};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::application::error::ApplicationError;

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tag {
    PHILOSOPHIE,
    INFORMATIQUE,
    SOCIOLOGIE,
    POLITIQUE,
    HISTOIRE
}



impl TryFrom<String> for Tag {
    type Error = ApplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "philosophie" => Ok(Tag::PHILOSOPHIE),
            "histoire" => Ok(Tag::HISTOIRE),
            "informatique" => Ok(Tag::INFORMATIQUE),
            "politique" => Ok(Tag::POLITIQUE),
            "sociologie" => Ok(Tag::SOCIOLOGIE),
            _ => Err(ApplicationError::DefaultError)
        }
    }
}


impl TryFrom<&str> for Tag {
    type Error = ApplicationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "philosophie" => Ok(Tag::PHILOSOPHIE),
            "histoire" => Ok(Tag::HISTOIRE),
            "informatique" => Ok(Tag::INFORMATIQUE),
            "politique" => Ok(Tag::POLITIQUE),
            "sociologie" => Ok(Tag::SOCIOLOGIE),
            _ => Err(ApplicationError::DefaultError)
        }
    }
}


impl TryFrom<&String> for Tag {
    type Error = ApplicationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return match value.to_lowercase().as_str() {
            "philosophie" => Ok(Tag::PHILOSOPHIE),
            "histoire" => Ok(Tag::HISTOIRE),
            "informatique" => Ok(Tag::INFORMATIQUE),
            "politique" => Ok(Tag::POLITIQUE),
            "sociologie" => Ok(Tag::SOCIOLOGIE),
            _ => Err(ApplicationError::DefaultError)
        }
    }
}




impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tag::PHILOSOPHIE => write!(f, "Philosophie"),
            Tag::HISTOIRE => write!(f,"Histoire"),
            Tag::INFORMATIQUE => write!(f, "Informatique"),
            Tag::POLITIQUE => write!(f, "Politique"),
            Tag::SOCIOLOGIE => write!(f, "Sociologie")
        }
    }
}

impl From<Tag> for String {
    fn from(val: Tag) -> Self {
        match val {
            Tag::PHILOSOPHIE => String::from("Philosophie"),
            Tag::HISTOIRE => String::from("Histoire"),
            Tag::INFORMATIQUE => String::from( "Informatique"),
            Tag::POLITIQUE => String::from("Politque"),
            Tag::SOCIOLOGIE => String::from("Sociologie")
        }
    }
}

