use crate::aabb::AABB;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};
use enum_dispatch::enum_dispatch;
use nalgebra::Point3;

pub mod bvh_node;
pub mod sphere;
pub mod triangle;

pub use bvh_node::*;
pub use sphere::*;
pub use triangle::*;

#[enum_dispatch]
#[derive(Clone)]
pub enum Geometry {
    Sphere,
    Triangle,
    BvhNode,
}

#[enum_dispatch(Geometry)]
pub trait GeometricObject {
    fn scale(&mut self, l: f64);
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn normal(&self, p: &Vec3) -> Vec3;
    fn get_center(&self) -> Vec3;
    fn get_min_point(&self) -> Point3<f64>;
    fn get_max_point(&self) -> Point3<f64>;
    fn get_bounding_box(&self) -> AABB;
    fn get_samples(&self, sample_points_sqrt: usize) -> Vec<Vec3>;
    fn get_material_id(&self) -> usize;
}
