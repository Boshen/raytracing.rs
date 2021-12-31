use crate::brdf::{Brdf, GlossySpecular, Lambertian, PerfectSpecular};
use crate::color::Color;
use crate::light::Light;
use crate::material::{shade, Material};
use crate::model::Vec3;
use crate::ray::{Hit, Ray};

pub struct Reflective {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
    pub specular_brdf: GlossySpecular,
    pub reflective_brdf: PerfectSpecular,
}

impl Reflective {
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
    fn shade(&self, hit: &Hit) -> Color {
        shade(self, hit)
    }

    fn emissive(&self) -> bool {
        false
    }

    fn ambient(&self, hit: &Hit) -> Color {
        self.ambient_brdf
            .rho()
            .component_mul(&hit.world.ambient_light.radiance(hit))
    }

    fn diffuse(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color {
        self.diffuse_brdf.f(hit, wo, wi)
    }

    fn specular(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color {
        self.specular_brdf.f(hit, wo, wi)
    }

    fn reflective(&self, hit: &Hit, wo: &Vec3) -> Color {
        let normal = hit.normal;
        let ndotwo = normal.dot(wo);
        let wi = normal * (2.0 * ndotwo) - wo;
        let fr = self.reflective_brdf.sample_f(hit, wo, &wi);
        let reflected_ray = Ray::new(hit.hit_point, wi);
        hit.world
            .trace(&reflected_ray, hit.depth + 1)
            .component_mul(&fr)
            * normal.dot(&wi)
    }
}
