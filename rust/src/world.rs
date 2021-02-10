use nalgebra::{Dot, Norm};
use num_traits::identities::Zero;
use std::collections::HashMap;

use crate::color::Color;
use crate::geometric_object::{BvhNode, GeometricObject};
use crate::light::{AmbientLight, LightEnum};
use crate::material::Material;
use crate::model::Vec3;
use crate::ray::{Ray, RayHit};
use crate::view_plane::ViewPlane;

pub struct World {
    pub vp: ViewPlane,
    pub lights: Vec<LightEnum>,
    pub bvh: BvhNode,
    pub ambient_light: AmbientLight,
    pub materials: HashMap<usize, Material>,
}

impl World {
    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth >= 15 {
            return Color::zero();
        }
        self.bvh
            .intersects(ray)
            .map_or(Color::zero(), |(distance, geometric_object)| {
                let hit_point = ray.get_point(distance);
                let normal = geometric_object.normal(&hit_point);
                let wo = (-1.0 * ray.dir).normalize();
                // revert normal if we hit the inside surface
                let adjusted_normal = normal * normal.dot(&wo).signum();
                let rayhit = RayHit {
                    ray,
                    hit_point,
                    geometric_object: &geometric_object,
                    world: &self,
                    normal: adjusted_normal,
                    depth,
                };
                self.get_material(geometric_object.get_material_id())
                    .shade(&rayhit)
            })
    }

    pub fn is_in_shadow<F>(&self, point: &Vec3, dir: &Vec3, test_distance: F) -> bool
    where
        F: Fn(f64) -> bool,
    {
        let shadow_ray = Ray::new(point + 0.00001 * dir, *dir);
        self.bvh
            .intersects(&shadow_ray)
            .filter(|(_, o)| {
                !matches!(
                    self.get_material(o.get_material_id()),
                    Material::Emissive(_)
                )
            })
            .map_or(false, |(dist, _)| test_distance(dist))
    }

    pub fn get_material(&self, material_id: usize) -> &Material {
        self.materials.get(&material_id).unwrap()
    }
}
