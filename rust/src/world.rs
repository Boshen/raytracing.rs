use nalgebra::{Dot, Norm};
use num_traits::identities::Zero;
use std::ops::{Add, Mul};

use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::model::{Model, Vec3};
use crate::ray::{Ray, RayHit};

pub struct World {
    pub width: u32,
    pub height: u32,
    pub lights: Vec<Box<dyn Light>>,
    pub models: Vec<Model>,
    pub ambient_light: Box<dyn Light>,
}

impl World {
    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth >= 15 {
            return Color::zero();
        }
        let intersection = self
            .models
            .iter()
            .filter(|model| model.aabb.intersects(&ray))
            .flat_map(|model| {
                model
                    .hittables
                    .iter()
                    .map(move |hittable| (model, hittable))
            })
            .filter_map(|(model, hittable)| {
                hittable.intersects(ray).map(|dist| (dist, model, hittable))
            })
            .min_by(|t1, t2| (t1.0).partial_cmp(&t2.0).expect("Tried to compare a NaN"));

        return intersection.map_or(Color::zero(), |(distance, model, hittable)| {
            let hit_point = ray.get_point(distance);

            let normal = hittable.normal(&hit_point);
            let wo = ray.dir.mul(-1.0).normalize();
            // revert normal if we hit the inside surface
            let adjusted_normal = normal.mul(normal.dot(&wo).signum());
            let rayhit = RayHit {
                ray,
                hit_point,
                material: &model.material,
                hittable: &hittable,
                world: &self,
                normal: adjusted_normal,
                depth,
            };
            return model.material.shade(&rayhit);
        });
    }

    pub fn is_in_shadow(
        &self,
        point: &Vec3,
        dir: &Vec3,
        test_distance: &dyn Fn(f64) -> bool,
    ) -> bool {
        let shadow_ray = Ray::new(point.add(dir.mul(0.00001)), *dir);
        return self
            .models
            .iter()
            .filter(|m| {
                if let Material::Emissive(_) = *m.material {
                    false
                } else {
                    true
                }
            })
            .filter(|m| m.aabb.intersects(&shadow_ray))
            .flat_map(|m| m.hittables.iter())
            .any(|h| h.intersects(&shadow_ray).map_or(false, test_distance));
    }
}
