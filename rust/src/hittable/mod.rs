use crate::model::Vec3;
use crate::ray::Ray;

pub mod sphere;
pub mod triangle;

pub use sphere::*;
pub use triangle::*;

pub trait Hittable: Send + Sync {
    fn scale(&mut self, l: f64) -> ();
    fn intersects(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, p: &Vec3) -> Vec3;
    fn get_min_point(&self) -> Vec3;
    fn get_max_point(&self) -> Vec3;
}
