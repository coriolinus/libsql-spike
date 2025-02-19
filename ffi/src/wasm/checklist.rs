use std::sync::Arc;

use wasm_bindgen::prelude::*;

use crate::{Db, Item, Result};

pub type ChecklistId = i64;

#[wasm_bindgen]
pub struct Checklist {
    inner: checklist::Checklist,
}

impl From<checklist::Checklist> for Checklist {
    fn from(inner: checklist::Checklist) -> Self {
        Self { inner }
    }
}

impl Checklist {
    pub(crate) fn wrap_arc(inner: checklist::Checklist) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}

#[wasm_bindgen]
impl Checklist {
    #[wasm_bindgen(constructor)]
    pub async fn new(db: &Db, name: &str) -> Result<Self> {
        checklist::Checklist::new(db, name)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn load(db: &Db, id: ChecklistId) -> Result<Option<Checklist>> {
        checklist::Checklist::load(db, id.into())
            .await
            .map(|option| option.map(Into::into))
            .map_err(Into::into)
    }

    pub async fn items(&self, db: &Db) -> Result<Vec<Item>> {
        self.inner
            .items(db)
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    pub async fn all(db: &Db) -> Result<Vec<Checklist>> {
        checklist::Checklist::all(db)
            .await
            .map(|ok| ok.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    pub async fn delete(db: &Db, id: ChecklistId) -> Result<()> {
        checklist::Checklist::delete(db, id.into())
            .await
            .map_err(Into::into)
    }

    pub fn id(&self) -> ChecklistId {
        self.inner.id.into()
    }

    pub fn name(&self) -> String {
        self.inner.name.clone()
    }
}
