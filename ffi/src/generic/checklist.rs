use std::sync::Arc;

use crate::{Db, Item, Result};

pub type ChecklistId = i64;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
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

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub async fn checklist_new(db: &Db, name: &str) -> Result<Checklist> {
    checklist::Checklist::new(db, name)
        .await
        .map(Into::into)
        .map_err(Into::into)
}

#[cfg_attr(feature = "uniffi", uniffi::export(name = "checklist_load"))]
pub async fn load(db: &Db, id: ChecklistId) -> Result<Option<Arc<Checklist>>> {
    checklist::Checklist::load(db, id.into())
        .await
        .map(|option| option.map(Checklist::wrap_arc))
        .map_err(Into::into)
}

#[cfg_attr(feature = "uniffi", uniffi::export(name = "checklist_all"))]
pub async fn all(db: &Db) -> Result<Vec<Arc<Checklist>>> {
    checklist::Checklist::all(db)
        .await
        .map(|ok| ok.into_iter().map(Checklist::wrap_arc).collect())
        .map_err(Into::into)
}

#[cfg_attr(feature = "uniffi", uniffi::export(name = "checklist_delete"))]
pub async fn delete(db: &Db, id: ChecklistId) -> Result<()> {
    checklist::Checklist::delete(db, id.into())
        .await
        .map_err(Into::into)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Checklist {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub async fn new(db: &Db, name: &str) -> Result<Self> {
        checklist_new(db, name).await
    }

    pub async fn items(&self, db: &Db) -> Result<Vec<Arc<Item>>> {
        self.inner
            .items(db)
            .await
            .map(|items| items.into_iter().map(Item::wrap_arc).collect())
            .map_err(Into::into)
    }

    pub fn id(&self) -> ChecklistId {
        self.inner.id.into()
    }

    pub fn name(&self) -> String {
        self.inner.name.clone()
    }
}
