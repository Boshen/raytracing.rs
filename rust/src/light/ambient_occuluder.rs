use nalgebra::{Cross, Norm};
use std::ops::{Add, Mul};

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
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
        let shadow_amount = get_hemisphere_sampler(self.sample_points_sqrt)
            .into_iter()
            .map(|sp| u.mul(sp.x).add(v.mul(sp.y)).add(w.mul(sp.z)).normalize())
            .filter(|dir| !hit.world.is_in_shadow(&hit.hit_point, &dir))
            .count();
        let sample_points = self.sample_points_sqrt * self.sample_points_sqrt;
        return self
            .cl
            .mul(self.ls)
            .mul((shadow_amount as f64) / (sample_points as f64));
    }

    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        let (u, v, w) = self.uvw(hit);
        return u.add(v).add(w);
    }
}
