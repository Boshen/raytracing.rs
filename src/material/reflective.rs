use super::Material;
use crate::brdf::{Brdf, GlossySpecular, Lambertian, PerfectSpecular};
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::{Hit, Ray};

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
        Self {
            ambient_brdf,
            diffuse_brdf,
            specular_brdf,
            reflective_brdf,
        }
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
        let wo = -hit.ray.dir;
        let normal = hit.normal;
        let ndotwo = normal.dot(&wo);
        let wi = normal * (2.0 * ndotwo) - wo;
        let fr = self.reflective_brdf.sample_f(hit, &wi);
        let reflected_ray = Ray::new(hit.hit_point, wi);
        hit.renderer
            .trace(&reflected_ray, hit.depth + 1)
            .component_mul(&fr)
            * normal.dot(&wi)
    }
}
