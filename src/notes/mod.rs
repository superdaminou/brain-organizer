mod note;
pub mod connecteur;

pub use note::Note;

use crate::application_error::ApplicationError;

pub trait ConnecteurNote {
    fn get_one(&self, id: &String) -> anyhow::Result<Note>;
    fn get_all(&self) -> anyhow::Result<Vec<Note>>;
    fn delete(&self, note: &String) -> anyhow::Result<()>;
    fn create(&self, note: &Note) -> anyhow::Result<(), anyhow::Error>;
    fn update(&self, note: &Note) -> anyhow::Result<(), ApplicationError>;
}