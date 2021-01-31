use nalgebra::{distance, Norm};
use num_traits::identities::Zero;
use std::ops::{Add, Div, Sub};

use crate::color::Color;
use crate::geometric_object::{GeometricObject, Geometry};
use crate::light::Light;
use crate::material::Emissive;
use crate::model::Vec3;
use crate::ray::RayHit;

pub struct AreaLight {
    center: Vec3,
    geometric_objects: Vec<Geometry>,
    sample_points_sqrt: u32,
    pub material: Emissive,
}

impl AreaLight {
    pub fn new(geometric_objects: Vec<Geometry>, material: Emissive) -> AreaLight {
        let center = geometric_objects
            .iter()
            .map(|h| h.get_center())
            .fold(Vec3::zero(), |a, b| a.add(b))
            .div(geometric_objects.len() as f64);
        return AreaLight {
            center,
            geometric_objects,
            sample_points_sqrt: 5,
            material,
        };
    }
}

impl Light for AreaLight {
    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        return self.center.sub(hit.hit_point).normalize();
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.material.radiance();
    }

    fn shadow_amount(&self, hit: &RayHit) -> f64 {
        let sqrt = self.sample_points_sqrt as f64;
        let weight = sqrt * sqrt * self.geometric_objects.len() as f64;
        let total = self
            .geometric_objects
            .iter()
            .flat_map(|t| t.get_samples(self.sample_points_sqrt))
            .filter(|point_on_light| {
                let wi = point_on_light.sub(hit.hit_point).normalize(); // light direction
                let d = distance(point_on_light.as_point(), hit.hit_point.as_point());
                return !hit.world.is_in_shadow(&hit.hit_point, &wi, &|t| t < d);
            })
            .count() as f64;
        return total / weight;
    }
}
