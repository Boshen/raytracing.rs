use nalgebra::Point3;

mod sphere;
mod triangle;

pub use sphere::*;
pub use triangle::*;

use crate::{
    aabb::Aabb,
    model::Vec3,
    ray::{HitRecord, Ray},
    sampler::Sampler,
};

pub trait Geometry: Send + Sync {
    fn scale(&mut self, _l: f64) {}

    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn normal(&self, _p: &Point3<f64>) -> Vec3 {
        Vec3::zeros()
    }

    fn get_center(&self) -> Point3<f64> {
        Point3::origin()
    }

    fn get_min_point(&self) -> Point3<f64>;

    fn get_max_point(&self) -> Point3<f64>;

    fn get_bounding_box(&self) -> Aabb {
        Aabb::new(self.get_min_point(), self.get_max_point())
    }

    fn get_samples(&self, _sampler: &Sampler) -> Vec<Point3<f64>> {
        vec![]
    }
}
