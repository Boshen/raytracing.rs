use std::f64::INFINITY;

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::RayHit;

use crate::sampler::get_hemisphere_sampler;

pub struct AmbientOcculuder {
    pub ls: f64,
    pub cl: Color,
    pub sample_points_sqrt: usize,
}

impl AmbientOcculuder {
    fn uvw(&self, hit: &RayHit) -> (Vec3, Vec3, Vec3) {
        let w = hit.normal;
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);
        (u, v, w)
    }
}

impl Light for AmbientOcculuder {
    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        let (u, v, w) = self.uvw(hit);
        u + v + w
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, hit: &RayHit) -> f64 {
        let (u, v, w) = self.uvw(hit);
        let sample_points = (self.sample_points_sqrt * self.sample_points_sqrt) as f64;
        let total = get_hemisphere_sampler(self.sample_points_sqrt)
            .map(|sp| (u * sp.x + v * sp.y + w * sp.z).normalize())
            .filter(|dir| !hit.world.is_in_shadow(&hit.hit_point, &dir, INFINITY))
            .count() as f64;
        total / sample_points
    }
}
