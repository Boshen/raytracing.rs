use super::Material;
use crate::{
    brdf::{Brdf, GlossySpecular, Lambertian},
    color::Color,
    model::Vec3,
    ray::{Hit, Ray},
};

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
        assert!(
            (0.0..1.0).contains(&(diffuse_brdf.kd + specular_brdf.ks)),
            "kd + ks >= 1.0 in Phong Constructor"
        );
        Self { ambient_brdf, diffuse_brdf, specular_brdf }
    }
}

impl Material for Phong {
    fn ambient(&self) -> Color {
        self.ambient_brdf.rho()
    }

    fn diffuse(&self, hit: &Hit, wi: &Vec3) -> Color {
        self.diffuse_brdf.f(hit, wi)
    }

    fn specular(&self, hit: &Hit, wi: &Vec3) -> Color {
        self.specular_brdf.f(hit, wi)
    }

    // FUTURE: Consider creating a separate GlossyReflector material type (Chapter 25)
    // This would allow for materials that have both Phong shading and perfect reflection
    // without mixing concerns in the Phong material itself.
    fn reflective(&self, hit: &Hit) -> Color {
        let mut wi = Vec3::zeros();
        let mut pdf = 0.0;
        let fr = self.specular_brdf.sample_f(hit, &mut wi, &mut pdf);
        let reflected_ray = Ray::new(hit.hit_point, wi);
        hit.renderer.trace(&reflected_ray, hit.depth + 1).component_mul(&fr) * hit.normal.dot(&wi)
            / pdf
    }
}
