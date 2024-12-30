mod note;
pub mod connecteur;

pub use note::Note;

use crate::application_error::ApplicationError;

pub trait ConnecteurNote {
    fn get_one(&self, id: &String) -> Result<Note, ApplicationError>;
    fn get_all(&self) -> Result<Vec<Note>, ApplicationError>;
    fn delete(&self, note: &String) -> Result<(), ApplicationError>;
    fn create(&self, note: &Note) -> Result<(), ApplicationError>;
    fn update(&self, note: &Note) -> Result<(), ApplicationError>;
}