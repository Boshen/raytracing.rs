use nalgebra::{distance, Point3};

use super::Light;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

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
        let b = hit.renderer.is_in_shadow(&hit.hit_point, &direction, d);
        f64::from(u32::from(!b))
    }

    fn set_sample_points_sqrt(&mut self, _n: u8) {}
}
