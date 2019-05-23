use crate::geometry::Pt;
use crate::model::Surface;


pub trait Light {
    fn brightness_at(&self, surface: &Surface) -> f64;
}


pub struct PointLight {
    point: Pt,
    brightness: f64,
}

impl PointLight {
    pub fn new(point: Pt, brightness: f64) -> PointLight {
        PointLight { point, brightness }
    }
}

impl Light for PointLight {
    fn brightness_at(&self, surface: &Surface) -> f64 {
        let line = self.point.sub(&surface.point);
        let dist = line.len();
        let cos = line.dot(&surface.normal) / dist;

        if cos > 0. {
            self.brightness * cos / dist.powi(2)
        } else {
            0.
        }
    }
}
