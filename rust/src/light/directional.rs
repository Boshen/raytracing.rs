use std::ops::Mul;

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::RayHit;

pub struct DirectionalLight {
    pub ls: f64,
    pub cl: Color,
    pub direction: Vec3,
}

impl Light for DirectionalLight {
    fn get_direction(&self, _hit: &RayHit) -> Vec3 {
        return self.direction;
    }

    fn shadow_amount(&self, _hit: &RayHit) -> f64 {
        return 1.0;
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.cl.mul(self.ls);
    }
}
