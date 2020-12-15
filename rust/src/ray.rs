use std::ops::Add;
use std::ops::Mul;

use crate::model::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn get_point(&self, distance: f64) -> Vec3 {
        return self.origin.add(self.dir.mul(distance));
    }
}
