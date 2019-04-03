use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Scene;

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Self {
        Scene
    }

    pub fn render(&self, image: &mut Image) {
        for x in 0 .. image.width {
            for y in 0 .. image.height {
                image.put(x, y, RGB(255, 0, 0));
            }
        }
    }
}


struct RGB(u8, u8, u8);


#[wasm_bindgen]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height * 4;
        let pixels = vec![0; size];
        Image { width, height, pixels }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    fn put(&mut self, x: usize, y: usize, color: RGB) {
        let RGB(r, g, b) = color;
        let idx = (x + y * self.width) * 4;

        for (i, &v) in [r, g, b, 255].iter().enumerate() {
            self.pixels[idx + i] = v;
        }
    }
}
