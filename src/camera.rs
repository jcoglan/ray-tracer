use crate::geometry::{self, Pt, Unit, Ray};


pub struct Camera {
    position: Pt,
    width: f64,
    length: f64,
}

impl Camera {
    pub fn new(position: Pt, width: f64, length: f64) -> Camera {
        Camera { position, width, length }
    }

    pub fn film(&self, width: usize, height: usize) -> Film {
        let eye = self.position.add(-self.length, &geometry::K);
        let center = self.position.clone();
        let axes = (geometry::I, geometry::J);
        let scale = self.width / 2.;
        let width = width as f64;
        let height = height as f64;

        Film { eye, center, axes, scale, width, height }
    }
}


pub struct Film {
    eye: Pt,
    center: Pt,
    axes: (Unit, Unit),
    scale: f64,
    width: f64,
    height: f64,
}

impl Film {
    pub fn ray_at(&self, x: usize, y: usize) -> Ray {
        self.eye.ray_to(self.point_at(x, y))
    }

    fn point_at(&self, x: usize, y: usize) -> Pt {
        let fx = (x as f64) / self.width - 0.5;
        let fy = (y as f64) / self.height - 0.5;

        let (dx, dy) = &self.axes;

        let sx = self.scale;
        let sy = sx * self.height / self.width;

        self.center.add(sx * fx, dx).add(sy * fy, dy)
    }
}
