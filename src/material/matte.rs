use super::Material;
use crate::brdf::{Brdf, Lambertian};
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

/// Perfect Diffuse Reflection with ambient and diffuse shading
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

    fn diffuse(&self, hit: &Hit, _wi: &Vec3) -> Color {
        let z = Color::zeros();
        self.diffuse_brdf.f(hit, &z)
    }

    // for path tracing (Chapter 26)
    // fn reflective(&self, hit: &Hit) -> Color {
    // let mut wi = Vec3::zeros();
    // let mut pdf = 0.0;
    // let fr = self.diffuse_brdf.sample_f(hit, &mut wi, &mut pdf);
    // let reflected_ray = Ray::new(hit.hit_point, wi);
    // hit.renderer
    // .trace(&reflected_ray, hit.depth + 1)
    // .component_mul(&fr)
    // * hit.normal.dot(&wi)
    // / pdf
    // }
}
