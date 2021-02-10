use crate::model::Vec3;
use crate::world::World;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn get_point(&self, distance: f64) -> Vec3 {
        self.origin + self.dir * distance
    }
}

pub struct HitRecord {
    pub dist: f64,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material_id: usize,
}

pub struct RayHit<'a> {
    pub ray: &'a Ray,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub world: &'a World,
    pub depth: i32,
    pub material_id: usize,
}
