use wasm_bindgen::prelude::*;
use crate::camera::{Camera, Film};
use crate::color::RGB;
use crate::geometry::Pt;
use crate::light::{Light, PointLight};
use crate::model::{Model, Surface, Sphere};


#[wasm_bindgen]
pub struct Scene {
    models: Vec<Box<dyn Model>>,
    lights: Vec<Box<dyn Light>>,
    camera: Camera,
}

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Scene {
        let models = Vec::new();
        let lights = Vec::new();
        let camera = Camera::new(Pt(0., 0., 0.), 10., 5.);
        Scene { models, lights, camera }
    }

    pub fn add_sphere(&mut self, center: Pt, radius: f64, color: RGB) {
        self.add_model(Sphere::new(center, radius, color));
    }

    fn add_model<M>(&mut self, model: M)
        where M: Model + 'static
    {
        self.models.push(Box::new(model));
    }

    pub fn add_point_light(&mut self, point: Pt, brightness: f64) {
        self.add_light(PointLight::new(point, brightness));
    }

    fn add_light<L>(&mut self, light: L)
        where L: Light + 'static
    {
        self.lights.push(Box::new(light));
    }

    pub fn render(&self, image: &mut Image) {
        let film = self.camera.film(image.width, image.height);

        for x in 0 .. image.width {
            for y in 0 .. image.height {
                let color = self.color_at(&film, x, y);
                image.put(x, y, color);
            }
        }
    }

    fn color_at(&self, film: &Film, x: usize, y: usize) -> RGB {
        let ray = film.ray_at(x, y);

        let nearest = self.models.iter()
            .flat_map(|model| model.intersect(&ray))
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        if let Some((_, surface)) = nearest {
            self.apply_lighting(surface)
        } else {
            self.bg_color(x, y)
        }
    }

    fn apply_lighting(&self, surface: Surface) -> RGB {
        let brightness = self.lights.iter()
            .map(|light| light.flux_at(&surface.point))
            .sum();

        surface.color.scale(brightness)
    }

    fn bg_color(&self, _: usize, _: usize) -> RGB {
        RGB(0x10, 0x10, 0x10)
    }
}


#[wasm_bindgen]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: usize, height: usize) -> Image {
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
