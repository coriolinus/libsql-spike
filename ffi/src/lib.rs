use std::{ops::Deref, sync::Arc};

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!("checklist_ffi");

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi", uniffi(flat_error))]
pub enum Error {
    #[error(transparent)]
    Inner(#[from] checklist::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct Db {
    inner: checklist::Db,
}

impl Db {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn new(path: &str, encryption_key: Vec<u8>) -> Result<Db> {
        let encryption_key = encryption_key.into();
        let encryption_config = checklist::EncryptionConfig {
            cipher: checklist::Cipher::Aes256Cbc,
            encryption_key,
        };
        checklist::Db::new(path, encryption_config)
            .await
            .map(|inner| Self { inner })
            .map_err(Into::into)
    }
}

impl Deref for Db {
    type Target = checklist::Db;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct Checklist {
    inner: checklist::Checklist,
}

impl From<checklist::Checklist> for Checklist {
    fn from(inner: checklist::Checklist) -> Self {
        Self { inner }
    }
}

pub type ChecklistId = i64;

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub async fn delete_checklist(db: &Db, id: ChecklistId) -> Result<()> {
    checklist::Checklist::delete(db, id.into())
        .await
        .map_err(Into::into)
}

impl Checklist {
    fn wrap_arc(inner: checklist::Checklist) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl Checklist {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn new(db: &Db, name: &str) -> Result<Self> {
        checklist::Checklist::new(db, name)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn load(db: &Db, id: ChecklistId) -> Result<Option<Arc<Self>>> {
        checklist::Checklist::load(db, id.into())
            .await
            .map(|option| option.map(Self::wrap_arc))
            .map_err(Into::into)
    }

    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn all(db: &Db) -> Result<Vec<Arc<Self>>> {
        checklist::Checklist::all(db)
            .await
            .map(|ok| ok.into_iter().map(Self::wrap_arc).collect())
            .map_err(Into::into)
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

pub type ItemId = i64;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct Item {
    inner: checklist::Item,
}

impl From<checklist::Item> for Item {
    fn from(inner: checklist::Item) -> Self {
        Self { inner }
    }
}

impl Item {
    fn wrap_arc(inner: checklist::Item) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub async fn delete_item(db: &Db, item_id: ItemId) -> Result<()> {
    checklist::Item::delete(db, item_id.into())
        .await
        .map_err(Into::into)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl Item {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn new(db: &Db, checklist_id: ChecklistId, item: &str) -> Result<Self> {
        checklist::Item::new(db, checklist_id.into(), item.to_owned())
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn load(db: &Db, item_id: ItemId) -> Result<Option<Arc<Self>>> {
        checklist::Item::load(db, item_id.into())
            .await
            .map(|option| option.map(Self::wrap_arc))
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
