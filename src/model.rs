use crate::geometry::{Pt, Ray};


pub trait Model {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Pt)>;
}


pub struct Sphere {
    center: Pt,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Pt, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Model for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Pt)> {
        let oc = self.center.sub(&ray.origin);
        let op_len = oc.dot(&ray.direction);

        let k = self.radius.powi(2) + op_len.powi(2) - oc.len().powi(2);
        if k < 0. { return None; }

        let d = op_len - k.sqrt();

        if d > 0. {
            Some((d, ray.point_at(d)))
        } else {
            None
        }
    }
}
