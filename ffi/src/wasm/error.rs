use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[wasm_bindgen]
pub struct Error(#[from] checklist::Error);
