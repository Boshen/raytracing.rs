use nalgebra::{Point3, distance};

use super::{Light, in_shadow};
use crate::{color::Color, model::Vec3, ray::Hit};

pub struct Point {
    pub ls: f64,
    pub cl: Color,
    pub location: Point3<f64>,
}

impl Light for Point {
    fn get_direction(&self, hit: &Hit) -> Vec3 {
        (self.location - hit.hit_point).normalize()
    }

    fn radiance(&self, _hit: &Hit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, hit: &Hit) -> f64 {
        let direction = (self.location - hit.hit_point).normalize();
        let d = distance(&self.location, &hit.hit_point);
        let b = in_shadow(hit, &direction, d);
        f64::from(u32::from(!b))
    }
}
