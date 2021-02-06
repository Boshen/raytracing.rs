use nalgebra::{Dot, Norm, PartialOrder};
use num_traits::identities::Zero;
use std::ops::{Add, Mul, Sub};

use crate::brdf::*;
use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::{Ray, RayHit};

pub mod emissive;
pub mod matte;
pub mod phong;
pub mod reflective;

pub use emissive::*;
pub use matte::*;
pub use phong::*;
pub use reflective::*;

pub enum Material {
    Matte(Matte),
    Phong(Phong),
    Reflective(Reflective),
    Emissive(Emissive),
}

impl Material {
    pub fn shade(&self, hit: &RayHit) -> Color {
        if let Material::Emissive(emissive) = self {
            return emissive.radiance();
        }

        let ambient_color = self.ambient_color(hit);
        hit.world
            .lights
            .iter()
            .map(|light| {
                // wi: incoming direction
                // ndotwi: angle between light and normal
                let wi = light.get_direction(hit);
                let ndotwi = hit.normal.dot(&wi);
                // not hit by light
                if ndotwi <= 0.0 {
                    return Color::zero();
                }

                let radiance = light.radiance(hit);
                if radiance.partial_le(&Vec3::zero()) {
                    return Color::zero();
                }

                // wo: reflected direction
                let shadow_amount = light.shadow_amount(&hit);

                let wo = hit.ray.dir.mul(-1.0).normalize();
                self.diffuse_color(hit, &wo, &wi)
                    .add(self.specular_color(hit, &wo, &wi))
                    .mul(radiance.mul(shadow_amount))
                    .mul(ndotwi)
                    .add(self.reflective_color(hit, &wo))
            })
            .fold(ambient_color, |a, b| a.add(b))
    }

    fn ambient_color(&self, hit: &RayHit) -> Color {
        let z = Vec3::zero();
        let rho = match self {
            Material::Matte(m) => m.diffuse_brdf.rho(),
            Material::Phong(m) => m.ambient_brdf.rho(),
            Material::Reflective(m) => m.ambient_brdf.rho(),
            Material::Emissive(_) => z,
        };
        rho.mul(hit.world.ambient_light.radiance(hit))
    }

    fn diffuse_color(&self, hit: &RayHit, wo: &Vec3, wi: &Vec3) -> Color {
        let z = Vec3::zero();
        match self {
            Material::Matte(m) => m.diffuse_brdf.f(hit, &z, &z),
            Material::Phong(m) => m.diffuse_brdf.f(hit, wo, wi),
            Material::Reflective(m) => m.diffuse_brdf.f(hit, wo, wi),
            Material::Emissive(_) => z,
        }
    }

    fn specular_color(&self, hit: &RayHit, wo: &Vec3, wi: &Vec3) -> Color {
        let z = Vec3::zero();
        match self {
            Material::Matte(_) => z,
            Material::Phong(m) => m.specular_brdf.f(hit, wo, wi),
            Material::Reflective(m) => m.specular_brdf.f(hit, wo, wi),
            Material::Emissive(_) => z,
        }
    }

    fn reflective_color(&self, hit: &RayHit, wo: &Vec3) -> Color {
        let z = Vec3::zero();
        match self {
            Material::Matte(_) => z,
            Material::Phong(_) => z,
            Material::Emissive(_) => z,
            Material::Reflective(m) => {
                let normal = hit.normal;
                let ndotwo = normal.dot(&wo);
                let wi = normal.mul(2.0 * ndotwo).sub(wo);
                let fr = m.reflective_brdf.sample_f(hit, &wo, &wi);
                let reflected_ray = Ray::new(hit.hit_point, wi);
                hit.world
                    .trace(&reflected_ray, hit.depth + 1)
                    .mul(fr)
                    .mul(normal.dot(&wi))
            }
        }
    }
}
