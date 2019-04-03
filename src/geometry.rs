use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Clone)]
pub struct Pt(pub f64, pub f64, pub f64);

#[wasm_bindgen]
impl Pt {
    pub fn new(x: f64, y: f64, z: f64) -> Pt {
        Pt(x, y, z)
    }
}

impl Pt {
    pub fn add(&self, len: f64, unit: &Unit) -> Pt {
        let Pt(x, y, z) = self;
        let Unit(dx, dy, dz) = unit;

        Pt(x + len * dx, y + len * dy, z + len * dz)
    }

    pub fn ray_to(&self, dst: Pt) -> Ray {
        let Pt(x1, y1, z1) = self;
        let Pt(x2, y2, z2) = dst;

        let origin = self.clone();
        let direction = Unit::new(x2 - x1, y2 - y1, z2 - z1);

        Ray { origin, direction }
    }

    pub fn sub(&self, other: &Pt) -> Pt {
        let Pt(x1, y1, z1) = self;
        let Pt(x2, y2, z2) = other;

        Pt(x1 - x2, y1 - y2, z1 - z2)
    }

    pub fn dot<T>(&self, other: T) -> f64
        where T: Into<(f64, f64, f64)>
    {
        let Pt(x1, y1, z1) = self;
        let (x2, y2, z2) = other.into();

        x1 * x2 + y1 * y2 + z1 * z2
    }

    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl From<&Pt> for (f64, f64, f64) {
    fn from(pt: &Pt) -> Self {
        let Pt(x, y, z) = pt;
        (*x, *y, *z)
    }
}


pub struct Unit(f64, f64, f64);

pub const I: Unit = Unit(1., 0., 0.);
pub const J: Unit = Unit(0., 1., 0.);
pub const K: Unit = Unit(0., 0., 1.);

impl Unit {
    fn new(i: f64, j: f64, k: f64) -> Unit {
        let m = (i*i + j*j + k*k).sqrt();
        Unit(i / m, j / m, k / m)
    }
}

impl From<&Unit> for (f64, f64, f64) {
    fn from(pt: &Unit) -> Self {
        let Unit(x, y, z) = pt;
        (*x, *y, *z)
    }
}


pub struct Ray {
    pub origin: Pt,
    pub direction: Unit,
}

impl Ray {
    pub fn point_at(&self, len: f64) -> Pt {
        self.origin.add(len, &self.direction)
    }
}
