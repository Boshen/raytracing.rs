use nalgebra::Point3;
use std::sync::Arc;

use crate::material::Material;
use crate::model::Vec3;
use crate::world::World;

pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vec3,
    pub inv_dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3<f64>, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            inv_dir: Vec3::new(dir.x.recip(), dir.y.recip(), dir.z.recip()),
        }
    }

    pub fn get_point(&self, distance: f64) -> Point3<f64> {
        self.origin + self.dir * distance
    }
}

pub struct HitRecord {
    pub dist: f64,
    pub hit_point: Point3<f64>,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

pub struct Hit<'a> {
    pub ray: &'a Ray,
    pub hit_point: Point3<f64>,
    pub normal: Vec3,
    pub world: &'a World,
    pub depth: i32,
    pub material: Arc<dyn Material>,
}
