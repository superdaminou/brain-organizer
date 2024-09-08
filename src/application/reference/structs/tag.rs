use std::fmt::{self};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::application::error::ApplicationError;

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tag {
    Philosophie,
    Informatique,
    Sociologie,
    Politique,
    Histoire,
    Economie,
    ExtremeDroite,
    Media
}



impl TryFrom<String> for Tag {
    type Error = ApplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}


impl TryFrom<&str> for Tag {
    type Error = ApplicationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "philosophie" => Ok(Tag::Philosophie),
            "histoire" => Ok(Tag::Histoire),
            "informatique" => Ok(Tag::Informatique),
            "politique" => Ok(Tag::Politique),
            "sociologie" => Ok(Tag::Sociologie),
            "economie" => Ok(Tag::Economie),
            "extreme_droite" => Ok(Tag::ExtremeDroite),
            "media" => Ok(Tag::Media),
            _ => Err(ApplicationError::EnumError(value.to_string()))
        }
    }
}


impl TryFrom<&String> for Tag {
    type Error = ApplicationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}




impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tag::Philosophie => write!(f, "Philosophie"),
            Tag::Histoire => write!(f,"Histoire"),
            Tag::Informatique => write!(f, "Informatique"),
            Tag::Politique => write!(f, "Politique"),
            Tag::Sociologie => write!(f, "Sociologie"),
            Tag::Economie => write!(f, "Economie"),
            Tag::ExtremeDroite =>  write!(f, "Extreme Droite"),
            Tag::Media =>  write!(f, "Media"),
        }
    }
}

impl From<Tag> for String {
    fn from(val: Tag) -> Self {
        match val {
            Tag::Philosophie => String::from("Philosophie"),
            Tag::Histoire => String::from("Histoire"),
            Tag::Informatique => String::from( "Informatique"),
            Tag::Politique => String::from("Politique"),
            Tag::Sociologie => String::from("Sociologie"),
            Tag::Economie => String::from("Economie"),
            Tag::ExtremeDroite => String::from("Extreme droite"),
            Tag::Media => String::from("Media"),
        }
    }
}

