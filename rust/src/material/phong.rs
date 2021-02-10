use crate::brdf::{GlossySpecular, Lambertian};

pub struct Phong {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
    pub specular_brdf: GlossySpecular,
}

impl Phong {
    pub fn new(
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
        specular_brdf: GlossySpecular,
    ) -> Phong {
        if diffuse_brdf.kd + specular_brdf.ks >= 1.0 {
            panic!("kd + ks >= 1.0 in Phong Constructor");
        }
        Phong {
            ambient_brdf,
            diffuse_brdf,
            specular_brdf,
        }
    }
}
