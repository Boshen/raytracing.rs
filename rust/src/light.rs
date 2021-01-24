use nalgebra::{Cross, Norm};
use std::ops::{Add, Mul, Sub};

use crate::model::{Color, Model, Vec3};
use crate::ray::{Ray, RayHit};
use crate::sampler::{get_hemisphere_sampler, get_unit_square_sampler};

pub trait Light: Send + Sync {
    fn radiance(&self, hit: &RayHit) -> Color;
    // the direction of the incoming light at a hit point
    fn get_direction(&self, hit: &RayHit) -> Vec3;
}

pub struct AmbientLight {
    pub ls: f64,   // radiance scaling factor [0, infinity)
    pub cl: Color, // light color
}

pub struct AmbientOcculuder {
    pub ls: f64,
    pub cl: Color,
    pub sample_points_sqrt: u32,
}

pub struct DirectionalLight {
    pub ls: f64,
    pub cl: Color,
    pub direction: Vec3,
}

pub struct PointLight {
    pub ls: f64,
    pub cl: Color,
    pub location: Vec3,
}

pub struct AreaLight {
    pub ls: f64,
    pub cl: Color,
    pub location: Vec3,
    pub width: f64,
    pub height: f64,
    pub sample_points_sqrt: u32,
}

impl Light for AmbientLight {
    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.cl.mul(self.ls);
    }

    fn get_direction(&self, _hit: &RayHit) -> Vec3 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}

impl AmbientOcculuder {
    fn uvw(&self, hit: &RayHit) -> (Vec3, Vec3, Vec3) {
        let w = hit.normal();
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);
        return (u, v, w);
    }
}

impl Light for AmbientOcculuder {
    fn radiance(&self, hit: &RayHit) -> Color {
        let (u, v, w) = self.uvw(hit);
        let sample_points = self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64;
        let shadow_amount = get_hemisphere_sampler(self.sample_points_sqrt)
            .into_iter()
            .map(|sp| {
                let dir = u.mul(sp.x).add(v.mul(sp.y)).add(w.mul(sp.z)).normalize();
                return if is_in_shadow(&hit.hit_point, &dir, &hit.scene.models) {
                    0.0
                } else {
                    1.0
                };
            })
            .sum::<f64>();
        return self.cl.mul(self.ls).mul(shadow_amount / sample_points);
    }

    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        let (u, v, w) = self.uvw(hit);
        return u.add(v).add(w);
    }
}

impl Light for DirectionalLight {
    fn radiance(&self, _hit: &RayHit) -> Color {
        return self.cl.mul(self.ls);
    }

    fn get_direction(&self, _hit: &RayHit) -> Vec3 {
        return self.direction;
    }
}

impl Light for PointLight {
    fn radiance(&self, hit: &RayHit) -> Color {
        let direction = self.location.sub(hit.hit_point).normalize();
        let shadow_amount = if is_in_shadow(&hit.hit_point, &direction, &hit.scene.models) {
            0.0
        } else {
            1.0
        };
        return self.cl.mul(self.ls).mul(shadow_amount);
    }

    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        return self.location.sub(hit.hit_point).normalize();
    }
}

impl Light for AreaLight {
    fn radiance(&self, hit: &RayHit) -> Color {
        let shadow_amount = self.intensity_at(&hit.hit_point, &hit.scene.models);
        return self.cl.mul(self.ls).mul(shadow_amount);
    }

    fn get_direction(&self, hit: &RayHit) -> Vec3 {
        return self.location.sub(hit.hit_point).normalize();
    }
}

impl AreaLight {
    fn intensity_at(&self, point: &Vec3, models: &Vec<Model>) -> f64 {
        let x = self.location.x - self.width / 2.0;
        let z = self.location.z - self.height / 2.0;
        return get_unit_square_sampler(self.sample_points_sqrt)
            .map(|(dx, dz)| {
                let new_location =
                    Vec3::new(x + dx * self.width, self.location.y, z + dz * self.width);
                let dir = new_location.sub(point).normalize();
                return if is_in_shadow(&point, &dir, &models) {
                    0.0
                } else {
                    1.0
                };
            })
            .sum::<f64>()
            / (self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }
}

fn is_in_shadow(point: &Vec3, dir: &Vec3, models: &Vec<Model>) -> bool {
    let shadow_ray = Ray {
        origin: point.add(dir.mul(0.00001)),
        dir: *dir,
    };
    return models
        .iter()
        .filter(|m| m.aabb.intersects(&shadow_ray))
        .flat_map(|m| m.hittables.iter())
        .any(|h| h.intersects(&shadow_ray).is_some());
}
