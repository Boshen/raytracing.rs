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

    /// TODO Chapter 26
    fn sample_f(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        Color::zeros()
    }
}
