//! Bounding Volume Hierarchy

use std::sync::Arc;

use nalgebra::Point3;
use rand::{Rng, rng};

use crate::{
    aabb::Aabb,
    geometric_object::Geometry,
    ray::{HitRecord, Ray},
};

pub struct Bvh {
    pub left: Arc<dyn Geometry>,
    pub right: Arc<dyn Geometry>,
    pub aabb: Aabb,
}

impl Bvh {
    /// # Panics
    ///
    /// * if `partial_cmp` fails
    #[must_use]
    pub fn construct(mut objects: Vec<Arc<dyn Geometry>>) -> Arc<dyn Geometry> {
        match objects.len() {
            0 => unreachable!(),
            1 => objects.remove(0),
            _ => {
                let axis = rng().random_range(0..3);
                objects.sort_by(|a, b| {
                    a.get_bounding_box().min[axis]
                        .partial_cmp(&b.get_bounding_box().min[axis])
                        .expect("legal partial_cmp")
                });
                let mut a = objects;
                let b = a.split_off(a.len() / 2);
                let left = Self::construct(a);
                let right = Self::construct(b);
                let aabb =
                    Aabb::get_surrounding_aabb(&left.get_bounding_box(), &right.get_bounding_box());
                Arc::new(Self { left, right, aabb })
            }
        }
    }
}

impl Geometry for Bvh {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.intersects(ray, t_min, t_max) {
            return None;
        }
        self.left.intersects(ray, t_min, t_max).map_or_else(
            || self.right.intersects(ray, t_min, t_max),
            |r1| self.right.intersects(ray, t_min, r1.dist).or(Some(r1)),
        )
    }

    fn get_min_point(&self) -> Point3<f64> {
        self.aabb.min
    }

    fn get_max_point(&self) -> Point3<f64> {
        self.aabb.max
    }
}
