use std::ops::Mul;

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Directional {
    pub ls: f64,
    pub cl: Color,
    pub direction: Vec3,
}

impl Light for Directional {
    fn get_direction(&self, _hit: &Hit) -> Vec3 {
        self.direction
    }

    fn shadow_amount(&self, _hit: &Hit) -> f64 {
        1.0
    }

    fn radiance(&self, _hit: &Hit) -> Color {
        self.cl.mul(self.ls)
    }

    fn set_sample_points_sqrt(&mut self, _n: u8) {}
}
