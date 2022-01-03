use super::Material;
use crate::brdf::{Brdf, Lambertian};
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Matte {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
}

impl Matte {
    #[must_use]
    pub const fn new(ambient_brdf: Lambertian, diffuse_brdf: Lambertian) -> Self {
        Self {
            ambient_brdf,
            diffuse_brdf,
        }
    }
}

impl Material for Matte {
    fn ambient(&self) -> Color {
        self.diffuse_brdf.rho()
    }

    fn diffuse(&self, hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        let z = Color::zeros();
        self.diffuse_brdf.f(hit, &z, &z)
    }
}
