use std::ops::Mul;
use std::ops::Add;

use crate::model::{Vec3};

pub struct Ray {
    pub start: Vec3,
    pub direction: Vec3
}

impl Ray {
  pub fn get_point(&self, distance: f64) -> Vec3 {
    return self.start.add(self.direction.mul(distance))
  }
}
