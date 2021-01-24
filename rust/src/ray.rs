use std::ops::Add;
use std::ops::Mul;

use crate::hittable::Hittable;
use crate::material::Material;
use crate::model::Vec3;
use crate::world::World;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        return Ray { origin, dir };
    }

    pub fn get_point(&self, distance: f64) -> Vec3 {
        return self.origin.add(self.dir.mul(distance));
    }
}

pub struct RayHit<'a> {
    pub ray: Box<&'a Ray>,
    pub hit_point: Vec3,
    pub material: Box<&'a Box<Material>>,
    pub hittable: &'a Box<dyn Hittable>,
    pub world: Box<&'a World>,
    pub normal: Vec3,
    pub depth: i32,
}
