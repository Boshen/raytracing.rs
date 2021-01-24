use std::ops::{Add, Mul};

use crate::model::{Color, Model, Vec3};
use crate::ray::{Ray, RayHit};

pub mod ambient;
pub mod ambient_occuluder;
pub mod area;
pub mod directional;
pub mod point;

pub use ambient::*;
pub use ambient_occuluder::*;
pub use area::*;
pub use directional::*;
pub use point::*;

pub trait Light: Send + Sync {
    fn radiance(&self, hit: &RayHit) -> Color;
    // the direction of the incoming light at a hit point
    fn get_direction(&self, hit: &RayHit) -> Vec3;
}

fn is_in_shadow(point: &Vec3, dir: &Vec3, models: &Vec<Model>) -> bool {
    let shadow_ray = Ray {
        origin: point.add(dir.mul(0.00001)),
        dir: *dir,
    };
    return models
        .iter()
        .filter(|m| m.aabb.intersects(&shadow_ray))
        .flat_map(|m| m.hittables.iter())
        .any(|h| h.intersects(&shadow_ray).is_some());
}
