use std::ops::Add;
use std::ops::Mul;

use crate::hittable::Hittable;
use crate::material::Material;
use crate::model::{Model, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn get_point(&self, distance: f64) -> Vec3 {
        return self.origin.add(self.dir.mul(distance));
    }
}

pub struct RayHit<'a> {
    pub ray: Box<&'a Ray>,
    pub hit_point: Vec3,
    pub material: Box<Material>,
    pub hittable: &'a Box<dyn Hittable>,
    pub models: Box<&'a Vec<Model>>,
}
