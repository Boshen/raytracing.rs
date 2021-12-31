use crate::brdf::Lambertian;

pub struct Matte {
    pub ambient_brdf: Lambertian,
    pub diffuse_brdf: Lambertian,
}

impl Matte {
    pub const fn new(ambient_brdf: Lambertian, diffuse_brdf: Lambertian) -> Self {
        Self {
            ambient_brdf,
            diffuse_brdf,
        }
    }
}
