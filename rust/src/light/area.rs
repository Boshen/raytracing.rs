use nalgebra::Norm;
use std::ops::{Mul, Sub};

use crate::color::Color;
use crate::light::Light;
use crate::material::Emissive;
use crate::model::Vec3;
use crate::ray::RayHit;
use crate::sampler::get_unit_square_sampler;

pub struct AreaLight {
    pub location: Vec3,
    pub width: f64,
    pub height: f64,
    pub sample_points_sqrt: u32,
    pub material: Emissive,
}

impl Light for AreaLight {
    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        return self.location.sub(hit.hit_point).normalize();
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.material.radiance();
    }

    fn shadow_amount(&self, hit: &RayHit) -> f64 {
        let x = self.location.x - self.width / 2.0;
        let z = self.location.z - self.height / 2.0;
        return get_unit_square_sampler(self.sample_points_sqrt)
            .map(|(dx, dz)| {
                let new_location =
                    Vec3::new(x + dx * self.width, self.location.y, z + dz * self.width);
                let dir = new_location.sub(hit.hit_point).normalize();
                return if hit.world.is_in_shadow(&hit.hit_point, &dir) {
                    0.0
                } else {
                    1.0
                };
            })
            .sum::<f64>()
            / (self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }
}
