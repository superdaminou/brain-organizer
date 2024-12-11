use anyhow::Context;
use reqwest::blocking::Body;
use crate::{application_error::ApplicationError, client, notes::{ConnecteurNote, Note}};


pub struct ConnecteurWebNote;

impl ConnecteurWebNote {
    pub fn new() -> ConnecteurWebNote {
        ConnecteurWebNote
    }
}

impl ConnecteurNote for ConnecteurWebNote {
    fn get_one(&self, id: &String) -> anyhow::Result<crate::notes::Note> {
        let path = format!("/notes/{}", id);
        client::get(&path).
            json::<Note>()
            .with_context(||"Error while deserializing get response".to_string())
    }

    fn get_all(&self) -> anyhow::Result<Vec<crate::notes::Note>> {
        let path = "/notes".to_string();
        client::get(&path)
            .json::<Vec<Note>>()
            .context("Error while creating reference")
    }

    fn delete(&self, note: &String) -> anyhow::Result<()> {
        let path = format!("/notes/{}", note);
        client::delete(&path)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .json::<()>()
            .context("Error while Deleting")
    }

    fn create(&self, note: &crate::notes::Note) -> anyhow::Result<(), anyhow::Error> {
        let path = "/notes".to_string();
        client::post(&path, Body::from(serde_json::to_string(note).unwrap()));
            
        Ok(())
    }

    fn update(&self, note: &crate::notes::Note) -> Result<(), ApplicationError> {
        let path = format!("/notes/{}", note.id);
        client::update(&path, Body::from(serde_json::to_string(note).unwrap()))
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .json::<()>()
            .map_err(|e|ApplicationError::DefaultError(e.to_string()))
    }
}