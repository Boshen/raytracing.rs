use nalgebra::{center, distance, Point3};
use std::sync::Arc;

use crate::color::Color;
use crate::geometric_object::Geometry;
use crate::light::Light;
use crate::material::Emissive;
use crate::model::Vec3;
use crate::ray::RayHit;

pub struct AreaLight {
    center: Point3<f64>,
    geometric_objects: Vec<Arc<dyn Geometry + Send + Sync>>,
    sample_points_sqrt: usize,
    pub material: Emissive,
}

impl AreaLight {
    pub fn new(
        geometric_objects: Vec<Arc<dyn Geometry + Send + Sync>>,
        material: Emissive,
    ) -> AreaLight {
        let center = geometric_objects
            .iter()
            .map(|o| o.get_center())
            .fold(Point3::origin(), |a, b| center(&a, &b));
        AreaLight {
            center,
            geometric_objects,
            sample_points_sqrt: 5,
            material,
        }
    }
}

impl Light for AreaLight {
    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        (self.center - hit.hit_point).normalize()
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        self.material.radiance()
    }

    fn shadow_amount(&self, hit: &RayHit) -> f64 {
        let sqrt = self.sample_points_sqrt as f64;
        let weight = sqrt * sqrt * self.geometric_objects.len() as f64;
        let total = self
            .geometric_objects
            .iter()
            .flat_map(|t| t.get_samples(self.sample_points_sqrt))
            .filter(|point_on_light| {
                let wi = (point_on_light - hit.hit_point).normalize(); // light direction
                let d = distance(point_on_light, &hit.hit_point);
                !hit.world.is_in_shadow(&hit.hit_point, &wi, d)
            })
            .count() as f64;
        total / weight
    }
}
