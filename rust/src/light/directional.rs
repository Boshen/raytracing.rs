use std::ops::Mul;

use crate::light::Light;
use crate::model::{Color, Vec3};
use crate::ray::RayHit;

pub struct DirectionalLight {
    pub ls: f64,
    pub cl: Color,
    pub direction: Vec3,
}

impl Light for DirectionalLight {
    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.cl.mul(self.ls);
    }

    fn get_direction(&self, _hit: &RayHit) -> Vec3 {
        return self.direction;
    }
}
