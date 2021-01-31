use crate::color::Color;
use crate::model::Vec3;
use crate::ray::RayHit;
use enum_dispatch::enum_dispatch;

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

#[enum_dispatch]
pub enum LightEnum {
    AmbientLight,
    AmbientOcculuder,
    AreaLight,
    DirectionalLight,
    PointLight,
}

#[enum_dispatch(LightEnum)]
pub trait Light: Send + Sync {
    // the direction of the incoming light at a hit point
    fn get_direction(&self, hit: &RayHit) -> Vec3;
    fn radiance(&self, hit: &RayHit) -> Color;
    fn shadow_amount(&self, hit: &RayHit) -> f64;
}
