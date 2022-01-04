use std::f64::INFINITY;

use super::Light;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct AmbientOcculuder {
    pub ls: f64,
    pub cl: Color,
}

impl AmbientOcculuder {
    #[must_use]
    pub const fn new(ls: f64, cl: Color) -> Self {
        Self { ls, cl }
    }

    #[must_use]
    pub fn uvw(hit: &Hit) -> (Vec3, Vec3, Vec3) {
        let w = hit.normal;
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);
        (u, v, w)
    }
}

impl Light for AmbientOcculuder {
    fn get_direction(&self, hit: &Hit) -> Vec3 {
        let (u, v, w) = Self::uvw(hit);
        u + v + w
    }

    fn radiance(&self, _hit: &Hit) -> Color {
        self.cl * self.ls
    }

    fn shadow_amount(&self, hit: &Hit) -> f64 {
        let (u, v, w) = Self::uvw(hit);
        let total = hit
            .renderer
            .sampler
            .hemisphere()
            .map(|sp| (u * sp.x + v * sp.y + w * sp.z).normalize())
            .filter(|dir| !hit.renderer.is_in_shadow(&hit.hit_point, dir, INFINITY))
            .count();
        #[allow(clippy::cast_possible_truncation)]
        (f64::from(total as u32) / f64::from(hit.renderer.sampler.count()))
    }
}
