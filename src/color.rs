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

impl RGB {
    pub fn scale(&self, amount: f64) -> RGB {
        let RGB(r, g, b) = *self;
        let amount = amount.min(1.);

        let r = ((r as f64) * amount) as u8;
        let g = ((g as f64) * amount) as u8;
        let b = ((b as f64) * amount) as u8;

        RGB(r, g, b)
    }
}
