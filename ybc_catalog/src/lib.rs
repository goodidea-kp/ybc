use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document: Document = window().ok_or("no window")?.document().ok_or("no document")?;
    let root: Element = document.get_element_by_id("root").ok_or("no #root element")?;
    root.set_inner_html("WASM app started (ybc_catalog). Replace this with your real app init.");
    Ok(())
}

