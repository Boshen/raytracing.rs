use nalgebra::{Vector3};
use std::ops::Mul;
use std::ops::Add;

pub struct Ray {
    pub start: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
  pub fn get_point(&self, distance: f64) -> Vector3<f64> {
    return self.start.add(self.direction.mul(distance))
  }
}
