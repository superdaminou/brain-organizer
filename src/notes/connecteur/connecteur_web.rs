use std::collections::HashSet;
use anyhow::{Context, Result};
use egui::ahash::{HashMap, HashMapExt};
use reqwest::{blocking::{Body, Response}, header::HeaderMap};
use uuid::Uuid;
use crate::{notes::ConnecteurNote, reference::{structs::reference::Reference, tag::Tag, ConnecteurReference, ModeTags}, server::SearchParams};


pub struct ConnecteurWebNote;

impl ConnecteurWebNote {
    pub fn new() -> ConnecteurWebNote {
        ConnecteurWebNote
    }
}

impl ConnecteurNote for ConnecteurWebNote {
    fn get_one(&self, id: &Uuid) -> anyhow::Result<crate::notes::Note> {
        todo!()
    }

    fn get_all(&self) -> anyhow::Result<Vec<crate::notes::Note>> {
        todo!()
    }

    fn delete(&self, note: &crate::notes::Note) -> anyhow::Result<()> {
        todo!()
    }

    fn create(&self, note: &crate::notes::Note) -> anyhow::Result<(), anyhow::Error> {
        todo!()
    }

    fn update(&self, note: &crate::notes::Note) -> anyhow::Result<()> {
        todo!()
    }
}