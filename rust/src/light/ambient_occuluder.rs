use nalgebra::{Cross, Norm};
use std::ops::{Add, Mul};

use crate::light::{is_in_shadow, Light};
use crate::model::{Color, Vec3};
use crate::ray::RayHit;

use crate::sampler::get_hemisphere_sampler;

pub struct AmbientOcculuder {
    pub ls: f64,
    pub cl: Color,
    pub sample_points_sqrt: u32,
}

impl AmbientOcculuder {
    fn uvw(&self, hit: &RayHit) -> (Vec3, Vec3, Vec3) {
        let w = hit.normal;
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);
        return (u, v, w);
    }
}

impl Light for AmbientOcculuder {
    fn radiance(&self, hit: &RayHit) -> Color {
        let (u, v, w) = self.uvw(hit);
        let sample_points = self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64;
        let shadow_amount = get_hemisphere_sampler(self.sample_points_sqrt)
            .into_iter()
            .map(|sp| {
                let dir = u.mul(sp.x).add(v.mul(sp.y)).add(w.mul(sp.z)).normalize();
                return if is_in_shadow(&hit.hit_point, &dir, &hit.scene.models) {
                    0.0
                } else {
                    1.0
                };
            })
            .sum::<f64>();
        return self.cl.mul(self.ls).mul(shadow_amount / sample_points);
    }

    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        let (u, v, w) = self.uvw(hit);
        return u.add(v).add(w);
    }
}
