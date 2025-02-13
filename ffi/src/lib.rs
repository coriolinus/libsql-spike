pub mod checklist;
pub mod item;

pub use checklist::{Checklist, ChecklistId};
pub use item::{Item, ItemId};

use ::checklist as libchecklist;
use std::ops::Deref;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!("checklist_ffi");

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi", uniffi(flat_error))]
pub enum Error {
    #[error(transparent)]
    Inner(#[from] libchecklist::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct Db {
    inner: libchecklist::Db,
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub async fn db_new(path: &str, encryption_key: Vec<u8>) -> Result<Db> {
    let encryption_key = encryption_key.into();
    let encryption_config = libchecklist::EncryptionConfig {
        cipher: libchecklist::Cipher::Aes256Cbc,
        encryption_key,
    };
    libchecklist::Db::new(path, encryption_config)
        .await
        .map(|inner| Db { inner })
        .map_err(Into::into)
}

#[uniffi::export]
impl Db {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn new(path: &str, encryption_key: Vec<u8>) -> Result<Db> {
        db_new(path, encryption_key).await
    }
}

impl Deref for Db {
    type Target = libchecklist::Db;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
