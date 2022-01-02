use super::Brdf;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct PerfectSpecular {
    pub kr: f64,   // reflection coefficient
    pub cr: Color, // reflection color
}

impl PerfectSpecular {
    #[must_use]
    pub const fn new(kr: f64, cr: Color) -> Self {
        Self { kr, cr }
    }
}

impl Brdf for PerfectSpecular {
    fn f(&self, _hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zeros() // is black for PerfectSpecular
    }

    fn rho(&self) -> Color {
        Color::zeros() // is black for PerfectSpecular
    }

    fn sample_f(&self, hit: &Hit, _wo: &Vec3, wi: &Vec3) -> Color {
        self.cr * self.kr / hit.normal.dot(wi)
    }
}
