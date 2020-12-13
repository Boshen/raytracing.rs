use nalgebra::{Vector3, Dot, Cross};
use std::ops::{Sub, Mul};

use crate::ray::{Ray};

pub struct Model {
    pub material: Material,
    pub hittables: Vec<Box<dyn Hittable>>,
}

#[derive(Copy, Clone)]
pub struct Material {
  pub diffuse_reflection: f64,
  pub diffuse_color: Vector3<f64>,
  pub reflection: f64,
  pub specular_refection: f64,
  pub shininess: f64,
  pub transparent: bool
}

pub trait Hittable {
    fn scale(&mut self, l: f64) -> ();
    fn intersects(&self, ray: &Ray) -> Option<f64>;
}

pub struct Triangle(
    pub Vector3<f64>,
    pub Vector3<f64>,
    pub Vector3<f64>
);

pub struct Sphere {
    pub radius: f64,
    pub center: Vector3<f64>
}

impl Model {
    pub fn new(material: Material, hittables:Vec<Box<dyn Hittable>>) -> Model {
        Model {
            material: material,
            hittables: hittables,
        }
    }

    pub fn scale(&mut self, l: f64) {
        for h in &mut self.hittables {
            h.scale(l);
        }
    }
}


impl Hittable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let epsilon = 0.000001;
        let e1 = self.1.sub(self.0);
        let e2 = self.2.sub(self.0);

        let h = ray.direction.cross(&e2);
        let a = e1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None
        }

        let f = 1.0 / a;
        let s = ray.start.sub(self.0);
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None
        }

        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = f * e2.dot(&q);
        if t <= epsilon {
            return None
        }

        return Some(t)
    }

    fn scale(&mut self, l: f64) {
        self.0 = self.0.mul(2.0 / l);
        self.1 = self.1.mul(2.0 / l);
        self.2 = self.2.mul(2.0 / l);

        self.0 = self.0.sub(Vector3::new(1.0, 1.0, 1.0));
        self.1 = self.1.sub(Vector3::new(1.0, 1.0, 1.0));
        self.2 = self.2.sub(Vector3::new(1.0, 1.0, 1.0));

        self.0.x = -self.0.x;
        self.1.x = -self.1.x;
        self.2.x = -self.2.x;

        self.0.y = -self.0.y;
        self.1.y = -self.1.y;
        self.2.y = -self.2.y;
    }
}

impl Sphere {
  pub fn new(radius: f64, center: Vector3<f64>) -> Sphere {
      Sphere {
          radius: radius,
          center: center
      }
  }
}

impl Hittable for Sphere {
  fn intersects(&self, ray: &Ray) -> Option<f64>{
    let center = self.center;
    let radius = self.radius;
    let start = ray.start;
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;

    let a = dx * dx + dy * dy + dz * dz;
    let b = 2.0 * dx * (start.x - center.x) + 2.0 * dy * (start.y - center.y) + 2.0 * dz * (start.z - center.z);
    let c =
      center.x * center.x +
      center.y * center.y +
      center.z * center.z +
      start.x * start.x +
      start.y * start.y +
      start.z * start.z -
      2.0 * (center.x * start.x + center.y * start.y + center.z * start.z) -
      radius * radius;

    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
      return None
    }

    let t = (-b - disc.sqrt()) / (2.0 * a);
    if t < 0.0 {
      return None
    }

    return Some(t)
  }

  fn scale(&mut self, l: f64) {
    self.center = self.center.mul(2.0 / l);
    self.center = self.center.sub(Vector3::new(1.0, 1.0, 1.0));
    self.center.x = -self.center.x;
    self.center.y = -self.center.y;
    self.radius = (self.radius * 2.0) / l;
  }
}
