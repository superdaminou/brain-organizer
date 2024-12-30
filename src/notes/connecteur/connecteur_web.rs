use reqwest::blocking::Body;
use crate::{application_error::ApplicationError, client, notes::{ConnecteurNote, Note}};


pub struct ConnecteurWebNote;

impl ConnecteurWebNote {
    pub fn new() -> ConnecteurWebNote {
        ConnecteurWebNote
    }
}

impl ConnecteurNote for ConnecteurWebNote {
    fn get_one(&self, id: &String) -> Result<crate::notes::Note, ApplicationError> {
        let path = format!("/notes/{}", id);
        client::get(&path)?
            .json::<Note>()
            .map_err(ApplicationError::from)
    }

    fn get_all(&self) -> Result<Vec<crate::notes::Note>, ApplicationError> {
        let path = "/notes".to_string();
        client::get(&path)?
            .json::<Vec<Note>>()
            .map_err(ApplicationError::from)
    }

    fn delete(&self, note: &String) -> Result<(), ApplicationError> {
        let path = format!("/notes/{}", note);
        client::delete(&path)?
            .json::<()>()
            .map_err(ApplicationError::from)
    }

    fn create(&self, note: &crate::notes::Note) -> Result<(), ApplicationError> {
        let path = "/notes".to_string();
        client::post(&path, Body::from(serde_json::to_string(note).unwrap()));
            
        Ok(())
    }

    fn update(&self, note: &crate::notes::Note) -> Result<(), ApplicationError> {
        let path = format!("/notes/{}", note.id);
        client::update(&path, Body::from(serde_json::to_string(note).unwrap()))?
            .json::<()>()
            .map_err(|e|ApplicationError::DefaultError(e.to_string()))
    }
}