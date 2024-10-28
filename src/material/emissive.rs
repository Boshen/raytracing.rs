use super::Material;
use crate::{color::Color, ray::Hit};

pub struct Emissive {
    pub ls: f64, // radiance scaling factor
    pub ce: Color,
}

impl Emissive {
    #[must_use]
    pub const fn new(ls: f64, ce: Color) -> Self {
        Self { ls, ce }
    }

    #[must_use]
    pub fn radiance(&self) -> Color {
        self.ce * self.ls
    }
}

impl Material for Emissive {
    fn shade(&self, _hit: &Hit) -> Color {
        self.radiance()
    }

    fn emissive(&self) -> bool {
        true
    }
}
