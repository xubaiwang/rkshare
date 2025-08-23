//! TODO: documentation

use wasm_bindgen::prelude::wasm_bindgen;

/// TODO: documentation
#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

/// TODO: documentation
#[wasm_bindgen]
pub async fn sse_summary() -> arrow_wasm::RecordBatch {
  let arrow = rkshare::sse::stock::summary::fetch::arrow::<()>()
    .await
    .unwrap();
  arrow_wasm::RecordBatch::new(arrow)
}
