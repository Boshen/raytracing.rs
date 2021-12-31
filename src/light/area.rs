use nalgebra::{center, distance, Point3};
use std::sync::Arc;

use crate::color::Color;
use crate::geometric_object::Geometry;
use crate::light::Light;
use crate::material::Emissive;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Area {
    center: Point3<f64>,
    geometric_objects: Vec<Arc<dyn Geometry>>,
    sample_points_sqrt: u8,
    pub material: Emissive,
}

impl Area {
    pub fn new(geometric_objects: Vec<Arc<dyn Geometry>>, material: Emissive) -> Self {
        let center = geometric_objects
            .iter()
            .map(|o| o.get_center())
            .fold(Point3::origin(), |a, b| center(&a, &b));
        Self {
            center,
            geometric_objects,
            sample_points_sqrt: 1,
            material,
        }
    }
}

impl Light for Area {
    fn get_direction(&self, hit: &Hit) -> Vec3 {
        (self.center - hit.hit_point).normalize()
    }

    fn radiance(&self, _hit: &Hit) -> Color {
        self.material.radiance()
    }

    fn shadow_amount(&self, hit: &Hit) -> f64 {
        let total = self
            .geometric_objects
            .iter()
            .flat_map(|t| t.get_samples(self.sample_points_sqrt))
            .filter(|point_on_light| {
                let wi = (point_on_light - hit.hit_point).normalize(); // light direction
                let d = distance(point_on_light, &hit.hit_point);
                !hit.world.is_in_shadow(&hit.hit_point, &wi, d)
            })
            .count();
        f64::from(total as u32)
            / f64::from(
                (self.sample_points_sqrt.pow(2) as usize * self.geometric_objects.len()) as u32,
            )
    }

    fn set_sample_points_sqrt(&mut self, n: u8) {
        self.sample_points_sqrt = n;
    }
}
