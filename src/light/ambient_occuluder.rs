use std::f64::INFINITY;

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::Hit;

use crate::sampler::get_hemisphere;

pub struct AmbientOcculuder {
    pub ls: f64,
    pub cl: Color,
    sample_points_sqrt: u8,
}

impl AmbientOcculuder {
    pub fn new(ls: f64, cl: Color) -> AmbientOcculuder {
        AmbientOcculuder {
            ls,
            cl,
            sample_points_sqrt: 1,
        }
    }

    pub fn uvw(hit: &Hit) -> (Vec3, Vec3, Vec3) {
        let w = hit.normal;
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);
        (u, v, w)
    }
}

impl Light for AmbientOcculuder {
    fn get_direction(&self, hit: &Hit) -> Vec3 {
        let (u, v, w) = AmbientOcculuder::uvw(hit);
        u + v + w
    }

    fn radiance(&self, _hit: &Hit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, hit: &Hit) -> f64 {
        let (u, v, w) = AmbientOcculuder::uvw(hit);
        let total = get_hemisphere(self.sample_points_sqrt)
            .map(|sp| (u * sp.x + v * sp.y + w * sp.z).normalize())
            .filter(|dir| !hit.world.is_in_shadow(&hit.hit_point, dir, INFINITY))
            .count();
        f64::from(total as u32) / f64::from(self.sample_points_sqrt * self.sample_points_sqrt)
    }

    fn set_sample_points_sqrt(&mut self, n: u8) {
        self.sample_points_sqrt = n;
    }
}
