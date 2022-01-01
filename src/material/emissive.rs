use crate::color::Color;
use crate::material::Material;
use crate::model::Vec3;
use crate::ray::Hit;

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

    fn ambient(&self, _hit: &Hit) -> Color {
        Color::zeros()
    }

    fn diffuse(&self, _hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    fn specular(&self, _hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    fn reflective(&self, _hit: &Hit, _wo: &Vec3) -> Color {
        Color::zeros()
    }
}
