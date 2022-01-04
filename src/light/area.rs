use nalgebra::{center, distance, Point3};
use std::sync::Arc;

use super::{in_shadow, Light};
use crate::color::Color;
use crate::geometric_object::Geometry;
use crate::material::Emissive;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct Area {
    center: Point3<f64>,
    geometric_objects: Vec<Arc<dyn Geometry>>,
    pub material: Emissive,
}

impl Area {
    #[must_use]
    pub fn new(geometric_objects: Vec<Arc<dyn Geometry>>, material: Emissive) -> Self {
        let center = geometric_objects
            .iter()
            .map(|o| o.get_center())
            .fold(Point3::origin(), |a, b| center(&a, &b));
        Self {
            center,
            geometric_objects,
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
            .flat_map(|t| t.get_samples(&hit.renderer.sampler))
            .filter(|point_on_light| {
                let wi = (point_on_light - hit.hit_point).normalize(); // light direction
                let d = distance(point_on_light, &hit.hit_point);
                !in_shadow(hit, &wi, d)
            })
            .count();
        #[allow(clippy::cast_possible_truncation)]
        (f64::from(total as u32)
            / f64::from(hit.renderer.sampler.count())
            / f64::from(self.geometric_objects.len() as u32))
    }
}
