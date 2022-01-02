mod setting;
mod simple;
mod thin_lens;

pub use setting::*;
pub use simple::*;
pub use thin_lens::*;

use nalgebra::Point2;

use crate::ray::Ray;

pub trait Camera: Send + Sync {
    fn setting(&self) -> &Setting;
    fn get_rays(&self, origin: Point2<f64>) -> Vec<Ray>;
}
