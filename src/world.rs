use nalgebra::Point3;
use std::f64::INFINITY;
use std::sync::Arc;

use crate::color::Color;
use crate::geometric_object::Geometry;
use crate::light::{Ambient, Light};
use crate::model::Vec3;
use crate::ray::{Hit, Ray};
use crate::view_plane::ViewPlane;

pub struct World {
    pub vp: ViewPlane,
    pub lights: Vec<Arc<dyn Light>>,
    pub bvh: Arc<dyn Geometry>,
    pub ambient_light: Ambient,
    pub max_depth: i32,
}

impl World {
    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth >= self.max_depth {
            return Color::zeros();
        }
        self.bvh
            .intersects(ray, 0.0, INFINITY)
            .map_or(Color::zeros(), |record| {
                let wo = (-1.0 * ray.dir).normalize();
                // revert normal if we hit the inside surface
                let adjusted_normal = record.normal * record.normal.dot(&wo).signum();
                let rayhit = Hit {
                    ray,
                    hit_point: record.hit_point,
                    material: record.material.clone(),
                    normal: adjusted_normal,
                    world: self,
                    depth,
                };
                record.material.shade(&rayhit)
            })
    }

    pub fn is_in_shadow(&self, point: &Point3<f64>, dir: &Vec3, t_max: f64) -> bool {
        let offset = 0.00001 * dir;
        let shadow_ray = Ray::new(point + offset, *dir);
        self.bvh
            .intersects(&shadow_ray, 0.0, t_max)
            .filter(|record| !record.material.emissive())
            .is_some()
    }
}
