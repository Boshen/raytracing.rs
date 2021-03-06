mod pinhole;
mod setting;
mod thin_lens;

pub use pinhole::*;
pub use setting::*;
pub use thin_lens::*;

use nalgebra::Point2;

use crate::ray::Ray;
use crate::sampler::Sampler;

pub trait Camera: Send + Sync {
    fn setting(&self) -> &Setting;
    fn get_rays(&self, origin: Point2<f64>, sampler: &Sampler) -> Vec<Ray>;
}
