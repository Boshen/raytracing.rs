use super::Light;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Ambient {
    pub ls: f64,   // radiance scaling factor [0, infinity)
    pub cl: Color, // light color
}

impl Light for Ambient {
    fn get_direction(&self, _hit: &Hit) -> Vec3 {
        Vec3::zeros()
    }

    fn radiance(&self, _hit: &Hit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, _hit: &Hit) -> f64 {
        1.0
    }

    fn set_sample_points_sqrt(&mut self, _n: u8) {}
}
