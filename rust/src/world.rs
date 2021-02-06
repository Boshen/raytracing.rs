use nalgebra::{Dot, Norm};
use num_traits::identities::Zero;
use std::ops::{Add, Mul};

use crate::color::Color;
use crate::geometric_object::GeometricObject;
use crate::light::{AmbientLight, LightEnum};
use crate::material::Material;
use crate::model::{Model, Vec3};
use crate::ray::{Ray, RayHit};

pub struct World {
    pub width: u32,
    pub height: u32,
    pub lights: Vec<LightEnum>,
    pub models: Vec<Model>,
    pub ambient_light: AmbientLight,
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
                    .geometric_objects
                    .iter()
                    .map(move |geometric_object| (model, geometric_object))
            })
            .filter_map(|(model, geometric_object)| {
                geometric_object
                    .intersects(ray)
                    .map(|dist| (dist, model, geometric_object))
            })
            .min_by(|t1, t2| (t1.0).partial_cmp(&t2.0).expect("Tried to compare a NaN"));

        intersection.map_or(Color::zero(), |(distance, model, geometric_object)| {
            let hit_point = ray.get_point(distance);

            let normal = geometric_object.normal(&hit_point);
            let wo = ray.dir.mul(-1.0).normalize();
            // revert normal if we hit the inside surface
            let adjusted_normal = normal.mul(normal.dot(&wo).signum());
            let rayhit = RayHit {
                ray,
                hit_point,
                material: &model.material,
                geometric_object,
                world: &self,
                normal: adjusted_normal,
                depth,
            };
            model.material.shade(&rayhit)
        })
    }

    pub fn is_in_shadow(
        &self,
        point: &Vec3,
        dir: &Vec3,
        test_distance: &dyn Fn(f64) -> bool,
    ) -> bool {
        let shadow_ray = Ray::new(point.add(dir.mul(0.00001)), *dir);
        self.models
            .iter()
            .filter(|m| !matches!(*m.material, Material::Emissive(_)))
            .filter(|m| m.aabb.intersects(&shadow_ray))
            .flat_map(|m| m.geometric_objects.iter())
            .any(|h| h.intersects(&shadow_ray).map_or(false, test_distance))
    }
}
