use super::Material;
use crate::{
    brdf::{Brdf, GlossySpecular, Lambertian, PerfectSpecular},
    color::Color,
    model::Vec3,
    ray::{Hit, Ray},
};

pub struct Reflective {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
    pub specular_brdf: GlossySpecular,
    pub reflective_brdf: PerfectSpecular,
}

impl Reflective {
    #[must_use]
    pub const fn new(
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
        specular_brdf: GlossySpecular,
        reflective_brdf: PerfectSpecular,
    ) -> Self {
        Self { ambient_brdf, diffuse_brdf, specular_brdf, reflective_brdf }
    }
}

impl Material for Reflective {
    fn ambient(&self) -> Color {
        self.ambient_brdf.rho()
    }

    fn diffuse(&self, hit: &Hit, wi: &Vec3) -> Color {
        self.diffuse_brdf.f(hit, wi)
    }

    fn specular(&self, hit: &Hit, wi: &Vec3) -> Color {
        self.specular_brdf.f(hit, wi)
    }

    fn reflective(&self, hit: &Hit) -> Color {
        let mut wi = Vec3::zeros();
        let mut pdf = 0.0;
        let fr = self.reflective_brdf.sample_f(hit, &mut wi, &mut pdf);
        let reflected_ray = Ray::new(hit.hit_point, wi);
        hit.renderer.trace(&reflected_ray, hit.depth + 1).component_mul(&fr) * hit.normal.dot(&wi)
            / pdf
    }
}
