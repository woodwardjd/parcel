extern crate parcel_js_swc_core;

use js_sys::Error;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform(config_val: JsValue) -> Result<JsValue, JsValue> {
  let config: parcel_js_swc_core::Config = from_value(config_val).map_err(JsValue::from)?;
  let result = parcel_js_swc_core::transform(config)
    .map_err(|e| Error::from(JsValue::from_str(&e.to_string())))?;
  to_value(&result).map_err(JsValue::from)
}
