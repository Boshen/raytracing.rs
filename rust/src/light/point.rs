use nalgebra::{distance, Point3};

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::RayHit;

pub struct PointLight {
    pub ls: f64,
    pub cl: Color,
    pub location: Point3<f64>,
}

impl Light for PointLight {
    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        (self.location - hit.hit_point).normalize()
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, hit: &RayHit) -> f64 {
        let direction = (self.location - hit.hit_point).normalize();
        let d = distance(&self.location, &hit.hit_point);
        let b = hit.world.is_in_shadow(&hit.hit_point, &direction, d);
        f64::from(u32::from(!b))
    }
}
