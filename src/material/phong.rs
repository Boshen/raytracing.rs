use super::{shade, Material};
use crate::brdf::{Brdf, GlossySpecular, Lambertian};
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Phong {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
    pub specular_brdf: GlossySpecular,
}

impl Phong {
    /// # Panics
    /// will panic if `diffuse_brdf.kd` + `specular_brdf.ks` >= 1.0
    #[must_use]
    pub fn new(
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
        specular_brdf: GlossySpecular,
    ) -> Self {
        if diffuse_brdf.kd + specular_brdf.ks >= 1.0 {
            panic!("kd + ks >= 1.0 in Phong Constructor");
        }
        Self {
            ambient_brdf,
            diffuse_brdf,
            specular_brdf,
        }
    }
}

impl Material for Phong {
    fn shade(&self, hit: &Hit) -> Color {
        shade(self, hit)
    }

    fn emissive(&self) -> bool {
        false
    }

    fn ambient(&self) -> Color {
        self.ambient_brdf.rho()
    }

    fn diffuse(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color {
        self.diffuse_brdf.f(hit, wo, wi)
    }

    fn specular(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color {
        self.specular_brdf.f(hit, wo, wi)
    }

    fn reflective(&self, _hit: &Hit, _wo: &Vec3) -> Color {
        Color::zeros()
    }
}
