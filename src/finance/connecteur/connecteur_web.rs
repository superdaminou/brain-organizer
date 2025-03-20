use reqwest::blocking::Body;
use crate::{application_error::ApplicationError, client, finance::{depense::Depense, ConnecteurDepense}};


pub struct ConnecteurWebDepense;

impl ConnecteurWebDepense {
    pub fn new() -> ConnecteurWebDepense {
        ConnecteurWebDepense
    }
}

impl ConnecteurDepense for ConnecteurWebDepense {
    fn get_one(&self, id: &String) -> Result<Depense, ApplicationError> {
        let path = format!("/depenses/{}", id);
        client::get(&path)?
            .json::<Depense>()
            .map_err(ApplicationError::from)
    }

    fn get_all(&self) -> Result<Vec<Depense>, ApplicationError> {
        let path = "/depenses".to_string();
        client::get(&path)?
            .json::<Vec<Depense>>()
            .map_err(ApplicationError::from)
    }

    fn delete(&self, depense: &String) -> Result<(), ApplicationError> {
        let path = format!("/depenses/{}", depense);
        client::delete(&path)?;

        Ok(())
    }

    fn create(&self, depense: &Depense) -> Result<(), ApplicationError> {
        let path = "/depenses".to_string();
        client::post(&path, Body::from(serde_json::to_string(depense).unwrap())).error_for_status().map_err(ApplicationError::from)?;
        Ok(())
    }

    fn update(&self, depense: &Depense) -> Result<(), ApplicationError> {
        let path = format!("/depenses/{}", depense.id.expect("Should have an id"));
        client::update(&path, Body::from(serde_json::to_string(depense).unwrap()))?
            .json::<()>()
            .map_err(ApplicationError::from)
    }
}