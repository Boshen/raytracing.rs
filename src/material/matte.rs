use crate::brdf::{Brdf, Lambertian};
use crate::color::Color;
use crate::light::Light;
use crate::material::shade;
use crate::material::Material;
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
    fn shade(&self, hit: &Hit) -> Color {
        shade(self, hit)
    }

    fn emissive(&self) -> bool {
        false
    }

    fn ambient(&self, hit: &Hit) -> Color {
        self.diffuse_brdf
            .rho()
            .component_mul(&hit.world.ambient_light.radiance(hit))
    }

    fn diffuse(&self, hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        let z = Color::zeros();
        self.diffuse_brdf.f(hit, &z, &z)
    }

    fn specular(&self, _hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    fn reflective(&self, _hit: &Hit, _wo: &Vec3) -> Color {
        Color::zeros()
    }
}
