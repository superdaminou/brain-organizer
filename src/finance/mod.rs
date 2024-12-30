use depense::Depense;

use crate::application_error::ApplicationError;

pub mod depense;
pub mod connecteur;


pub trait ConnecteurDepense {
    fn get_one(&self, id: &String) -> Result<Depense, ApplicationError>;
    fn get_all(&self) -> Result<Vec<Depense>, ApplicationError>;
    fn delete(&self, note: &String) -> Result<(), ApplicationError>;
    fn create(&self, note: &Depense) -> Result<(), ApplicationError>;
    fn update(&self, note: &Depense) -> Result<(), ApplicationError>;
}