#![allow(non_snake_case)]
use crate::core::{
    enums::{file_type::*, item_type::ItemType, view_mode::ViewMode},
    models::items::Items,
};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "invoke", catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn GetItems(path: &str, view_mode: ViewMode) -> Result<Vec<Items>, String> {
    let args = to_value(&serde_json::json!({ "path": path, "viewMode": view_mode }))
        .map_err(|e| e.to_string())?;

    let result = invoke("open_folder", args)
        .await
        .map_err(|e| e.as_string().unwrap_or_else(|| format!("{:?}", e)))?;

    let items: Vec<Items> = from_value(result).map_err(|e| e.to_string())?;

    Ok(items)
}

pub async fn GetDefaultStartPath() -> String {
    let result = invoke("get_default_start_path", JsValue::NULL).await;
    match result {
        Ok(v) => from_value(v).unwrap_or_default(),
        Err(e) => String::new(),
    }
}
