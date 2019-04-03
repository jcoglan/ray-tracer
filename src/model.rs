use crate::color::RGB;
use crate::geometry::{Pt, Ray};


pub trait Model {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Surface)>;
}


pub struct Surface {
    pub point: Pt,
    pub color: RGB,
}


pub struct Sphere {
    center: Pt,
    radius: f64,
    color: RGB,
}

impl Sphere {
    pub fn new(center: Pt, radius: f64, color: RGB) -> Sphere {
        Sphere { center, radius, color }
    }
}

impl Model for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Surface)> {
        let oc = self.center.sub(&ray.origin);
        let op_len = oc.dot(&ray.direction);

        let k = self.radius.powi(2) + op_len.powi(2) - oc.len().powi(2);
        if k < 0. { return None; }

        let d = op_len - k.sqrt();

        if d > 0. {
            let point = ray.point_at(d);
            let color = self.color.clone();
            Some((d, Surface { point, color }))
        } else {
            None
        }
    }
}
