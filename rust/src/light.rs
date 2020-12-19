use nalgebra::{Cross, Dot, Norm};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use crate::model::{Color, Hittable, Material, Model, Vec3};
use crate::ray::Ray;
use crate::sampler::{get_hemisphere_sampler, get_unit_square_sampler};

pub struct AmbientLight {
    pub radiance: f64,
    pub color: Color,
}

pub struct AmbientOcculuder {
    pub radiance: f64,
    pub color: Color,
    pub sample_points_sqrt: u32,
}

pub struct DirectionalLight {
    pub radiance: f64,
    pub color: Color,
    pub location: Vec3,
}

pub struct AreaLight {
    pub radiance: f64,
    pub color: Color,
    pub location: Vec3,
    pub width: f64,
    pub height: f64,
    pub sample_points_sqrt: u32,
}

pub enum Light {
    Ambient(AmbientLight),
    AmbientOcculuder(AmbientOcculuder),
    Directional(DirectionalLight),
    Area(AreaLight),
}

impl Light {
    pub fn shade(
        &self,
        ray: &Ray,
        point: &Vec3,
        model: &Model,
        hittable: &Box<dyn Hittable>,
        models: &Vec<Model>,
    ) -> Color {
        match self {
            Light::Ambient(l) => l.shade(&model.material),
            Light::AmbientOcculuder(l) => l.shade(&point, &hittable, &models),
            Light::Directional(l) => l.shade(&model.material, &point, &hittable),
            Light::Area(l) => l.shade(&model.material, &ray, &point, &hittable, &models),
        }
    }
}

impl AmbientLight {
    pub fn shade(&self, material: &Material) -> Color {
        let kd = material.diffuse_reflection;
        let cd = material.diffuse_color;
        let cl = self.color;
        let ls = self.radiance;
        return cd.mul(kd).mul(cl.mul(ls));
    }
}

impl AmbientOcculuder {
    pub fn shade(&self, point: &Vec3, hittable: &Box<dyn Hittable>, models: &Vec<Model>) -> Color {
        let w = hittable.normal(point);
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);

        let amount = get_hemisphere_sampler(self.sample_points_sqrt)
            .into_iter()
            .map(|sp| {
                let dir = u.mul(sp.x).add(v.mul(sp.y)).add(w.mul(sp.z)).normalize();
                if is_in_shadow(&point, &dir, &models) {
                    return 0.0;
                } else {
                    return 1.0;
                }
            })
            .sum::<f64>()
            / (self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);

        return self.color.mul(self.radiance * amount);
    }
}

impl DirectionalLight {
    pub fn shade(&self, material: &Material, point: &Vec3, hittable: &Box<dyn Hittable>) -> Color {
        let l = self.location.sub(point).normalize();
        let kd = material.diffuse_reflection;
        let cd = material.diffuse_color;
        let n = hittable.normal(point);
        let cl = self.color;
        let ls = self.radiance;
        return cd
            .mul(kd)
            .mul(1.0 / 3.14)
            .mul(n.dot(&l).max(0.0))
            .mul(cl.mul(ls));
    }
}

impl AreaLight {
    pub fn shade(
        &self,
        material: &Material,
        ray: &Ray,
        point: &Vec3,
        hittable: &Box<dyn Hittable>,
        models: &Vec<Model>,
    ) -> Color {
        let w = ray.origin.sub(point).normalize();
        let l = self.location.sub(point).normalize();
        let kd = material.diffuse_reflection;
        let cd = material.diffuse_color;
        let ks = material.specular_refection;
        let e = material.shininess;
        let n = hittable.normal(point);
        let cl = self.color;
        let ls = self.radiance;

        let mut shadow_intensity = 1.0;
        if n.dot(&w) > 0.0 {
            shadow_intensity = self.intensity_at(&point, &models)
        }

        if shadow_intensity <= 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // diffuse
        let diffuse_amount = n.dot(&l).max(0.0);
        let diffuse = cd
            .mul(kd)
            .mul(1.0 / 3.14)
            .mul(diffuse_amount)
            .mul(cl.mul(ls));

        // specular
        let r = n.mul(2.0 * diffuse_amount).sub(l);
        let specular_amount = r.dot(&w);
        let specular = cl.mul(ks * specular_amount.powf(e) * diffuse_amount * ls);
        return diffuse.add(specular).mul(shadow_intensity);
    }

    fn intensity_at(&self, point: &Vec3, models: &Vec<Model>) -> f64 {
        let x = self.location.x - self.width / 2.0;
        let z = self.location.z - self.height / 2.0;
        return get_unit_square_sampler(self.sample_points_sqrt)
            .map(|(dx, dz)| {
                let new_location =
                    Vec3::new(x + dx * self.width, self.location.y, z + dz * self.width);
                let dir = new_location.sub(point).normalize();
                if is_in_shadow(&point, &dir, &models) {
                    return 0.0;
                } else {
                    return 1.0;
                }
            })
            .sum::<f64>()
            / (self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }
}

fn is_in_shadow(point: &Vec3, dir: &Vec3, models: &Vec<Model>) -> bool {
    let shadow_ray = Ray {
        origin: point.add(dir.mul(0.00001)),
        dir: *dir,
    };
    for m in models.iter() {
        if !m.material.transparent {
            if m.aabb.intersects(&shadow_ray) {
                for h in m.hittables.iter() {
                    if let Some(_) = h.intersects(&shadow_ray) {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}
