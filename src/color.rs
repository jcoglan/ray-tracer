use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

#[wasm_bindgen]
impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB(r, g, b)
    }
}
