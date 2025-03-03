mod ambient;
mod ambient_occuluder;
mod area;
mod directional;
mod point;

pub use ambient::*;
pub use ambient_occuluder::*;
pub use area::*;
pub use directional::*;
pub use point::*;

use crate::{
    color::Color,
    model::Vec3,
    ray::{Hit, Ray},
};

pub trait Light: Send + Sync {
    // the direction of the incoming light at a hit point
    fn get_direction(&self, hit: &Hit) -> Vec3;
    fn radiance(&self, hit: &Hit) -> Color;
    fn shadow_amount(&self, hit: &Hit) -> f64;
}

#[must_use]
pub fn in_shadow(hit: &Hit, dir: &Vec3, tmax: f64) -> bool {
    let offset = 0.00001 * dir;
    let shadow_ray = Ray::new(hit.hit_point + offset, *dir);
    hit.renderer
        .scene
        .intersects(&shadow_ray, 0.0, tmax)
        .filter(|obj| !obj.material.emissive())
        .is_some()
}
