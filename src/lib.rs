use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn time_wasm(time: &str) -> String {
    format!("Time processed from js -> rust -> js: {}!", time)
}
