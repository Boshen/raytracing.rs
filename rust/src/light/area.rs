use nalgebra::{distance, Norm};
use num_traits::identities::Zero;
use std::ops::{Add, Div, Sub};

use crate::color::Color;
use crate::hittable::Triangle;
use crate::light::Light;
use crate::material::Emissive;
use crate::model::Vec3;
use crate::ray::RayHit;
use crate::sampler::get_triangle_sampler;

pub struct AreaLight {
    location: Vec3,
    triangles: Vec<Triangle>,
    sample_points_sqrt: u32,
    pub material: Emissive,
}

impl AreaLight {
    pub fn new(triangles: Vec<Triangle>, material: Emissive) -> AreaLight {
        let points = triangles
            .iter()
            .flat_map(|t| vec![t.0, t.1, t.2])
            .collect::<Vec<Vec3>>();
        let location = points
            .iter()
            .fold(Vec3::zero(), |a, b| a.add(b))
            .div(points.len() as f64);
        return AreaLight {
            location,
            triangles,
            sample_points_sqrt: 5,
            material,
        };
    }
}

impl Light for AreaLight {
    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        return self.location.sub(hit.hit_point).normalize();
    }

    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.material.radiance();
    }

    fn shadow_amount(&self, hit: &RayHit) -> f64 {
        let sqrt = self.sample_points_sqrt as f64;
        let weight = sqrt * sqrt * self.triangles.len() as f64;
        let total = self
            .triangles
            .iter()
            .flat_map(|t| get_triangle_sampler(self.sample_points_sqrt, t))
            .filter(|v| {
                let dir = v.sub(hit.hit_point).normalize();
                let d = distance(&self.location.to_point(), &hit.hit_point.to_point());
                return !hit.world.is_in_shadow(&hit.hit_point, &dir, &|t| t < d);
            })
            .count() as f64;
        return total / weight;
    }
}
