#[cfg(not(feature = "wasm"))]
mod generic;

#[cfg(not(feature = "wasm"))]
pub use generic::*;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::*;

#[cfg(all(feature = "wasm", feature = "uniffi"))]
compile_error!(
    "can't build this crate for uniffi and wasm simultaneously; their Error types are incompatible"
);

use ::checklist as libchecklist;
use std::ops::Deref;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!("checklist_ffi");

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
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

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Db {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
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
