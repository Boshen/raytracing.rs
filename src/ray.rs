use nalgebra::Point3;

use crate::model::Vec3;
use crate::world::World;

pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3<f64>, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn get_point(&self, distance: f64) -> Point3<f64> {
        self.origin + self.dir * distance
    }
}

pub struct HitRecord {
    pub dist: f64,
    pub hit_point: Point3<f64>,
    pub normal: Vec3,
    pub material_id: usize,
}

pub struct RayHit<'a> {
    pub ray: &'a Ray,
    pub hit_point: Point3<f64>,
    pub normal: Vec3,
    pub world: &'a World,
    pub depth: i32,
    pub material_id: usize,
}
