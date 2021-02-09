use nalgebra::{Dot, Norm};
use num_traits::identities::Zero;

use crate::color::Color;
use crate::geometric_object::GeometricObject;
use crate::light::{AmbientLight, LightEnum};
use crate::material::Material;
use crate::model::{Model, Vec3};
use crate::ray::{Ray, RayHit};
use crate::view_plane::ViewPlane;

pub struct World {
    pub vp: ViewPlane,
    pub lights: Vec<LightEnum>,
    pub models: Vec<Model>,
    pub ambient_light: AmbientLight,
}

impl World {
    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth >= 15 {
            return Color::zero();
        }
        self.models
            .iter()
            .flat_map(|model| model.intersects(ray).map(|(dist, o)| (dist, model, o)))
            .min_by(|t1, t2| (t1.0).partial_cmp(&t2.0).unwrap())
            .map_or(Color::zero(), |(distance, model, geometric_object)| {
                let hit_point = ray.get_point(distance);
                let normal = geometric_object.normal(&hit_point);
                let wo = (-1.0 * ray.dir).normalize();
                // revert normal if we hit the inside surface
                let adjusted_normal = normal * normal.dot(&wo).signum();
                let rayhit = RayHit {
                    ray,
                    hit_point,
                    material: &model.material,
                    geometric_object: &geometric_object,
                    world: &self,
                    normal: adjusted_normal,
                    depth,
                };
                model.material.shade(&rayhit)
            })
    }

    pub fn is_in_shadow<F>(&self, point: &Vec3, dir: &Vec3, test_distance: F) -> bool
    where
        F: Fn(f64) -> bool,
    {
        let shadow_ray = Ray::new(point + 0.00001 * dir, *dir);
        self.models
            .iter()
            .filter(|m| !matches!(*m.material, Material::Emissive(_)))
            .any(|m| {
                m.intersects(&shadow_ray)
                    .map_or(false, |(dist, _)| test_distance(dist))
            })
    }
}
