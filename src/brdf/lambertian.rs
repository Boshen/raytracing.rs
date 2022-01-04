use std::f64::consts::FRAC_1_PI;

use super::Brdf;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Lambertian {
    /// diffuse reflection coefficient [0, 1]
    pub kd: f64,

    /// diffuse color
    pub cd: Color,
}

impl Lambertian {
    #[must_use]
    pub const fn new(kd: f64, cd: Color) -> Self {
        Self { kd, cd }
    }
}

impl Brdf for Lambertian {
    /// Chapter 13.9
    fn f(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        self.rho() * FRAC_1_PI
    }

    /// Chapter 13.9
    fn rho(&self) -> Color {
        self.cd * self.kd
    }

    /// Chapter 26
    fn sample_f(&self, hit: &Hit, wi: &mut Vec3, pdf: &mut f64) -> Color {
        let w = hit.normal;
        let v = Vec3::new(0.0034, 1.0, 0.0071).cross(&w).normalize();
        let u = v.cross(&w);
        let sp = hit
            .renderer
            .sampler
            .hemisphere()
            .take(1)
            .collect::<Vec<_>>()
            .remove(0);
        *wi = (sp.x * u + sp.y * v + sp.z * w).normalize();
        *pdf = hit.normal.dot(wi) * FRAC_1_PI;
        self.kd * self.cd * FRAC_1_PI
    }
}
