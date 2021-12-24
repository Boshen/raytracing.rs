use nalgebra::{Point3, Translation3};
use std::{collections::HashMap, f64::INFINITY};

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
    pub materials: HashMap<usize, Box<Material>>,
}

impl World {
    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth >= 15 {
            return Color::zeros();
        }
        self.bvh
            .intersects(ray, 0.0, INFINITY)
            .map_or(Color::zeros(), |record| {
                let wo = (-1.0 * ray.dir).normalize();
                // revert normal if we hit the inside surface
                let adjusted_normal = record.normal * record.normal.dot(&wo).signum();
                let rayhit = RayHit {
                    ray,
                    hit_point: record.hit_point,
                    material_id: record.material_id,
                    normal: adjusted_normal,
                    world: self,
                    depth,
                };
                self.get_material(record.material_id).shade(&rayhit)
            })
    }

    pub fn is_in_shadow(&self, point: &Point3<f64>, dir: &Vec3, t_max: f64) -> bool {
        let offset = 0.00001 * dir;
        let shadow_ray = Ray::new(Translation3::from(offset) * point, *dir);
        self.bvh
            .intersects(&shadow_ray, 0.0, t_max)
            .filter(|record| {
                !matches!(self.get_material(record.material_id), Material::Emissive(_))
            })
            .is_some()
    }

    pub fn get_material(&self, material_id: usize) -> &Material {
        self.materials.get(&material_id).unwrap()
    }
}
