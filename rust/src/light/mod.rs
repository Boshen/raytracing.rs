use crate::model::{Color, Vec3};
use crate::ray::RayHit;

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
