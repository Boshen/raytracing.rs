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

use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub trait Light: Send + Sync {
    // the direction of the incoming light at a hit point
    fn get_direction(&self, hit: &Hit) -> Vec3;
    fn radiance(&self, hit: &Hit) -> Color;
    fn shadow_amount(&self, hit: &Hit) -> f64;
}
