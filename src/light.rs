use crate::geometry::Pt;


pub trait Light {
    fn flux_at(&self, point: &Pt) -> f64;
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
    fn flux_at(&self, point: &Pt) -> f64 {
        let dist = self.point.sub(point).len();
        self.brightness / dist.powi(2)
    }
}
