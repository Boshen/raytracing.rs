use nalgebra::{Vector3, Dot, Norm};
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Add;

use crate::model::{Model, Hittable};
use crate::ray::{Ray};

pub trait Light {
  fn shade(&self, ray: &Ray, point: Vector3<f64>, model: &Model, hittable: &Box<dyn Hittable>) -> Vector3<f64>;
}

pub struct AmbientLight {
    pub radiance: f64,
    pub color: Vector3<f64>
}

pub struct DirectionalLight {
    pub radiance: f64,
    pub color: Vector3<f64>,
    pub location: Vector3<f64>,
}

pub struct PointLight {
    pub radiance: f64,
    pub color: Vector3<f64>,
    pub location: Vector3<f64>,
}

impl Light for AmbientLight {
  fn shade(&self, _ray: &Ray, _point: Vector3<f64>, model: &Model, _hittable: &Box<dyn Hittable>) -> Vector3<f64> {
        let kd = model.material.diffuse_reflection;
        let cd = model.material.diffuse_color;
        let cl = self.color;
        let ls = self.radiance;
        return cd.mul(kd).mul(cl.mul(ls))
    }
}

impl Light for DirectionalLight {
  fn shade(&self, _ray: &Ray, point: Vector3<f64>, model: &Model, hittable: &Box<dyn Hittable>) -> Vector3<f64> {
    let l = self.location.sub(point).normalize();
    let kd = model.material.diffuse_reflection;
    let cd = model.material.diffuse_color;
    let n = hittable.normal(point);
    let cl = self.color;
    let ls = self.radiance;
    return cd
      .mul(kd)
      .mul(1.0 / 3.14)
      .mul(n.dot(&l).max(0.0))
      .mul(cl.mul(ls))
    }
}

impl Light for PointLight {
    fn shade(&self, ray: &Ray, point: Vector3<f64>, model: &Model, hittable: &Box<dyn Hittable>) -> Vector3<f64> {
        let w = ray.start.sub(point).normalize();
        let l = self.location.sub(point).normalize();
        let kd = model.material.diffuse_reflection;
        let cd = model.material.diffuse_color;
        let ks = model.material.specular_refection;
        let e = model.material.shininess;
        let n = hittable.normal(point);
        let cl = self.color;
        let ls = self.radiance;

        // diffuse
        let diffuse_amount = n.dot(&l).max(0.0);
        let diffuse = cd
          .mul(kd)
          .mul(1.0 / 3.14)
          .mul(diffuse_amount)
          .mul(cl.mul(ls));

        // specular
        let r = n.mul(2.0 * diffuse_amount).sub(l);
        let specular_amount = r.dot(&w);
        let specular = cl.mul(ks * specular_amount.powf(e) * diffuse_amount * ls);
        return diffuse.add(specular)
    }
}
