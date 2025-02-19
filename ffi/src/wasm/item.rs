use std::sync::Arc;

use wasm_bindgen::prelude::*;

use crate::{ChecklistId, Db, Result};

pub type ItemId = i64;

#[wasm_bindgen]
pub struct Item {
    inner: checklist::Item,
}

impl From<checklist::Item> for Item {
    fn from(inner: checklist::Item) -> Self {
        Self { inner }
    }
}

impl Item {
    pub(crate) fn wrap_arc(inner: checklist::Item) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}

#[wasm_bindgen]
impl Item {
    pub async fn new(db: &Db, checklist_id: ChecklistId, item: &str) -> Result<Self> {
        checklist::Item::new(db, checklist_id.into(), item.to_owned())
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn load(db: &Db, item_id: ItemId) -> Result<Option<Item>> {
        checklist::Item::load(db, item_id.into())
            .await
            .map(|option| option.map(Into::into))
            .map_err(Into::into)
    }

    pub async fn delete(db: &Db, item_id: ItemId) -> Result<()> {
        checklist::Item::delete(db, item_id.into())
            .await
            .map_err(Into::into)
    }

    pub async fn is_set(&self, db: &Db) -> Result<bool> {
        self.inner.is_set(db).await.map_err(Into::into)
    }

    pub async fn set_checked(&self, db: &Db, checked: bool) -> Result<()> {
        self.inner
            .set_checked(db, checked)
            .await
            .map_err(Into::into)
    }

    pub fn id(&self) -> ItemId {
        self.inner.id.into()
    }

    pub fn checklist_id(&self) -> ChecklistId {
        self.inner.checklist.into()
    }

    pub fn item(&self) -> String {
        self.inner.item.clone()
    }
}
