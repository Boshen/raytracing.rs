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
    fn sample_f(&self, hit: &Hit, _wo: &Vec3, wi: &Vec3) -> Color {
        self.cr * self.kr / hit.normal.dot(wi)
    }
}
