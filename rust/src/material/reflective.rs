use crate::brdf::{GlossySpecular, Lambertian, PerfectSpecular};

pub struct Reflective {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
    pub specular_brdf: GlossySpecular,
    pub reflective_brdf: PerfectSpecular,
}

impl Reflective {
    pub fn new(
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
        specular_brdf: GlossySpecular,
        reflective_brdf: PerfectSpecular,
    ) -> Reflective {
        return Reflective {
            ambient_brdf,
            diffuse_brdf,
            specular_brdf,
            reflective_brdf,
        };
    }
}
