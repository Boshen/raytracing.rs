use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::RayHit;

pub struct AmbientLight {
    pub ls: f64,   // radiance scaling factor [0, infinity)
    pub cl: Color, // light color
}

impl Light for AmbientLight {
    fn get_direction(&self, _hit: &RayHit) -> Vec3 {
        Vec3::zeros()
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, _hit: &RayHit) -> f64 {
        1.0
    }
}
