use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

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
    // the direction of the incoming light at a hit point
    fn get_direction(&self, hit: &Hit) -> Vec3;
    fn radiance(&self, hit: &Hit) -> Color;
    fn shadow_amount(&self, hit: &Hit) -> f64;
    fn set_sample_points_sqrt(&mut self, n: u8);
}
