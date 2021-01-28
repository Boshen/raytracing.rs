use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::RayHit;
use std::ops::Mul;

pub struct AmbientLight {
    pub ls: f64,   // radiance scaling factor [0, infinity)
    pub cl: Color, // light color
}

impl Light for AmbientLight {
    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.cl.mul(self.ls);
    }

    fn get_direction(&self, _hit: &RayHit) -> Vec3 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}
