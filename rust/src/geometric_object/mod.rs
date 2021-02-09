use crate::aabb::AABB;
use crate::model::Vec3;
use crate::ray::Ray;
use enum_dispatch::enum_dispatch;
use nalgebra::Point3;

pub mod sphere;
pub mod triangle;

pub use sphere::*;
pub use triangle::*;

#[enum_dispatch]
#[derive(Copy, Clone)]
pub enum Geometry {
    Sphere,
    Triangle,
}

#[enum_dispatch(Geometry)]
pub trait GeometricObject: Send + Sync {
    fn scale(&mut self, l: f64);
    fn intersects(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, p: &Vec3) -> Vec3;
    fn get_center(&self) -> Vec3;
    fn get_min_point(&self) -> Point3<f64>;
    fn get_max_point(&self) -> Point3<f64>;
    fn get_bounding_box(&self) -> AABB;
    fn get_samples(&self, sample_points_sqrt: usize) -> Vec<Vec3>;
}
