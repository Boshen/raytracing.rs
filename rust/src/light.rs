use nalgebra::{Vector3, Dot, Norm};
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Add;

use crate::model::{Model, Hittable, Material};
use crate::ray::{Ray};

pub struct LightData {
    pub radiance: f64,
    pub color: Vector3<f64>,
    pub location: Vector3<f64>
}

pub enum Light {
    Ambient(LightData),
    Directional(LightData),
    Point(LightData),
}

impl Light {
  pub fn shade(&self, ray: &Ray, point: &Vector3<f64>, model: &Model, hittable: &Box<dyn Hittable>, models: &Vec<Model>) -> Vector3<f64> {
      match self {
          Light::Ambient(l) => Light::shade_ambient(&l, &model.material),
          Light::Directional(l) => Light::shade_directional(&l, &model.material, &point, &hittable),
          Light::Point(l) => Light::shade_point(&l, &model.material, &ray, &point, &hittable, &models),
      }

    }

  fn shade_ambient(light: &LightData, material: &Material) -> Vector3<f64> {
        let kd = material.diffuse_reflection;
        let cd = material.diffuse_color;
        let cl = light.color;
        let ls = light.radiance;
        return cd.mul(kd).mul(cl.mul(ls))
  }

  fn shade_directional(light: &LightData, material: &Material, point: &Vector3<f64>, hittable: &Box<dyn Hittable>) -> Vector3<f64> {
      let l = light.location.sub(point).normalize();
      let kd = material.diffuse_reflection;
      let cd = material.diffuse_color;
      let n = hittable.normal(point);
      let cl = light.color;
      let ls = light.radiance;
      return cd
          .mul(kd)
          .mul(1.0 / 3.14)
          .mul(n.dot(&l).max(0.0))
          .mul(cl.mul(ls))
  }

  fn shade_point(light: &LightData, material: &Material, ray: &Ray, point: &Vector3<f64>, hittable: &Box<dyn Hittable>, models: &Vec<Model>) -> Vector3<f64> {
        let w = ray.start.sub(point).normalize();
        let l = light.location.sub(point).normalize();
        let kd = material.diffuse_reflection;
        let cd = material.diffuse_color;
        let ks = material.specular_refection;
        let e = material.shininess;
        let n = hittable.normal(point);
        let cl = light.color;
        let ls = light.radiance;

        if n.dot(&w) > 0.0 && Light::is_in_shadow(l, &point, &models) {
            return Vector3::new(0.0, 0.0, 0.0)
        }

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

    fn is_in_shadow(l: Vector3<f64>, point: &Vector3<f64>, models: &Vec<Model>) -> bool {
        let shadow_ray = Ray {start: point.add(l.mul(0.00001)), direction: l};
        for m in models.iter() {
            if !m.material.transparent {
                for h in m.hittables.iter() {
                    if let Some(_) = h.intersects(&shadow_ray) {
                        return true
                    }
                }
            }
        }
        return false
    }

}
