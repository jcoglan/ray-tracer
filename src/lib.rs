use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene;

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Self {
        Scene
    }

    pub fn hello(&self) -> String {
        String::from("hello world")
    }
}
