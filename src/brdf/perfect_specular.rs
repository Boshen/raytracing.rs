use super::Brdf;
use crate::{color::Color, model::Vec3, ray::Hit};

pub struct PerfectSpecular {
    /// reflection coefficient
    pub kr: f64,

    /// reflection color
    pub cr: Color,
}

impl PerfectSpecular {
    #[must_use]
    pub const fn new(kr: f64, cr: Color) -> Self {
        Self { kr, cr }
    }
}

impl Brdf for PerfectSpecular {
    /// Chapter 24
    fn sample_f(&self, hit: &Hit, wi: &mut Vec3, pdf: &mut f64) -> Vec3 {
        let wo = -hit.ray.dir;
        let ndotwo = hit.normal.dot(&wo);
        *wi = hit.normal * (2.0 * ndotwo) - wo;
        *pdf = hit.normal.dot(wi);
        self.cr * self.kr
    }
}
