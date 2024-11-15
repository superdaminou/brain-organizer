use strum_macros::Display;

pub mod service;
pub mod structs;
pub mod tag;


#[derive(serde::Deserialize, serde::Serialize, Default, Display, Clone, Copy, PartialEq, Eq)]
pub enum ModeTags {
    #[default]
    #[strum(to_string = "Inclus")]
    INCLUS,
    #[strum(to_string = "Exclus")]
    EXCLUS
}