mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    // NEW CONSOLE SERVICE
    wasm_logger::init(wasm_logger::Config::default());

    Ok(())
}
