use super::Brdf;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

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
    fn sample_f(&self, hit: &Hit, wi: &Vec3) -> Vec3 {
        self.cr * self.kr / hit.normal.dot(wi)
    }
}
