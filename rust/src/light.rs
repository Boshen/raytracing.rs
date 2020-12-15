use nalgebra::{Dot, Norm};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use crate::model::{Color, Hittable, Material, Model, Vec3};
use crate::ray::Ray;

pub struct AmbientLight {
    pub radiance: f64,
    pub color: Color,
}

pub struct DirectionalLight {
    pub radiance: f64,
    pub color: Color,
    pub location: Vec3,
}

pub struct PointLight {
    pub radiance: f64,
    pub color: Color,
    pub location: Vec3,
}

pub enum Light {
    Ambient(AmbientLight),
    Directional(DirectionalLight),
    Point(PointLight),
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
            Light::Directional(l) => l.shade(&model.material, &point, &hittable),
            Light::Point(l) => l.shade(&model.material, &ray, &point, &hittable, &models),
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

impl PointLight {
    pub fn shade(
        &self,
        material: &Material,
        ray: &Ray,
        point: &Vec3,
        hittable: &Box<dyn Hittable>,
        models: &Vec<Model>,
    ) -> Color {
        let w = ray.start.sub(point).normalize();
        let l = self.location.sub(point).normalize();
        let kd = material.diffuse_reflection;
        let cd = material.diffuse_color;
        let ks = material.specular_refection;
        let e = material.shininess;
        let n = hittable.normal(point);
        let cl = self.color;
        let ls = self.radiance;

        if n.dot(&w) > 0.0 && self.is_in_shadow(&l, &point, &models) {
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
        return diffuse.add(specular);
    }

    fn is_in_shadow(&self, l: &Vec3, point: &Vec3, models: &Vec<Model>) -> bool {
        let shadow_ray = Ray {
            start: point.add(l.mul(0.00001)),
            direction: *l,
        };
        for m in models.iter() {
            if !m.material.transparent {
                for h in m.hittables.iter() {
                    if let Some(_) = h.intersects(&shadow_ray) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
