use nalgebra::Point3;

mod bvh_node;
mod sphere;
mod triangle;

pub use bvh_node::*;
pub use sphere::*;
pub use triangle::*;

use crate::aabb::Aabb;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};
use crate::sampler::Sampler;

pub trait Geometry: Send + Sync {
    fn scale(&mut self, l: f64);
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn normal(&self, p: &Point3<f64>) -> Vec3;
    fn get_center(&self) -> Point3<f64>;
    fn get_min_point(&self) -> Point3<f64>;
    fn get_max_point(&self) -> Point3<f64>;
    fn get_bounding_box(&self) -> Aabb;
    fn get_samples(&self, sampler: &Sampler) -> Vec<Point3<f64>>;
}
