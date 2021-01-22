use nalgebra::Dot;
use std::ops::{Add, Mul, Sub};

use crate::brdf::BRDF;
use crate::model::{Color, Vec3};
use crate::ray::{Ray, RayHit};

pub enum Material {
    Matte(Matte),
    Phong(Phong),
    Reflective(Reflective),
}

pub struct Matte {
    pub ambient_brdf: Box<dyn BRDF>,
    pub diffuse_brdf: Box<dyn BRDF>,
}

pub struct Phong {
    pub ambient_brdf: Box<dyn BRDF>,
    pub diffuse_brdf: Box<dyn BRDF>,
    pub specular_brdf: Box<dyn BRDF>,
}

pub struct Reflective {
    pub ambient_brdf: Box<dyn BRDF>,
    pub diffuse_brdf: Box<dyn BRDF>,
    pub specular_brdf: Box<dyn BRDF>,
    pub reflective_brdf: Box<dyn BRDF>,
}

impl Material {
    pub fn shade(&self, hit: &RayHit) -> Color {
        let ambient_color = (match self {
            Material::Matte(m) => m.diffuse_brdf.rho(),
            Material::Phong(m) => m.ambient_brdf.rho(),
            Material::Reflective(m) => m.ambient_brdf.rho(),
        })
        .mul(hit.scene.ambient_light.radiance());

        return hit
            .scene
            .lights
            .iter()
            .filter_map(|light| {
                let wi = light.get_direction(hit);
                let ndotwi = hit.normal().dot(&wi);
                if ndotwi <= 0.0 {
                    return None;
                }

                let z = Vec3::new(0.0, 0.0, 0.0);
                let wo = hit.ray.dir.mul(-1.0);
                let normal = hit.normal();

                return light.shadow_intensity(hit).map(|shadow_intensity| {
                    let diffuse_f;
                    let mut specular_f = z;
                    let mut reflected_radiance = z;

                    match self {
                        Material::Matte(m) => {
                            diffuse_f = m.diffuse_brdf.f(hit, &z, &z);
                        }
                        Material::Phong(m) => {
                            diffuse_f = m.diffuse_brdf.f(hit, &wo, &wi);
                            specular_f = m.specular_brdf.f(hit, &wo, &wi);
                        }
                        Material::Reflective(m) => {
                            diffuse_f = m.diffuse_brdf.f(hit, &wo, &wi);
                            specular_f = m.specular_brdf.f(hit, &wo, &wi);

                            let ndotwo = normal.dot(&wo);
                            let wi = normal.mul(2.0 * ndotwo).sub(wo);
                            let fr = m.reflective_brdf.sample_f(hit, &wo, &wi);
                            let reflected_ray = Ray {
                                origin: hit.hit_point,
                                dir: wi,
                            };
                            reflected_radiance = hit
                                .scene
                                .trace(&reflected_ray, hit.depth + 1)
                                .mul(fr)
                                .mul(normal.dot(&wi));
                        }
                    };

                    return diffuse_f
                        .add(specular_f)
                        .mul(light.radiance())
                        .mul(ndotwi)
                        .mul(shadow_intensity)
                        .add(reflected_radiance);
                });
            })
            .fold(ambient_color, |a, b| a.add(b));
    }
}
