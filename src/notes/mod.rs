pub mod service;
mod note;
pub mod connecteur;

pub use note::Note;
use uuid::Uuid;

pub trait ConnecteurNote {
    fn get_one(&self, id: &Uuid) -> anyhow::Result<Note>;
    fn get_all(&self) -> anyhow::Result<Vec<Note>>;
    fn delete(&self, note: &Note) -> anyhow::Result<()>;
    fn create(&self, note: &Note) -> anyhow::Result<(), anyhow::Error>;
    fn update(&self, note: &Note) -> anyhow::Result<()>;
}