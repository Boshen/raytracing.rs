use nalgebra::{Cross, Dot, Norm};
use std::ops::{Add, Mul, Sub};

use crate::model::{Color, Model, Vec3};
use crate::ray::{Ray, RayHit};
use crate::sampler::{get_hemisphere_sampler, get_unit_square_sampler};

pub trait Light: Send + Sync {
    fn shade(&self, hit: &RayHit) -> Color;
    fn radiance(&self) -> Color;
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

pub struct PointLight {
    pub ls: f64,
    pub cl: Color,
}

pub struct DirectionalLight {
    pub ls: f64,
    pub cl: Color,
    pub direction: Vec3,
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
    fn radiance(&self) -> Color {
        return self.cl.mul(self.ls);
    }

    fn shade(&self, hit: &RayHit) -> Color {
        let kd = hit.material.diffuse_reflection;
        let cd = hit.material.diffuse_color;
        return cd.mul(kd).mul(self.radiance());
    }
}

impl Light for AmbientOcculuder {
    fn radiance(&self) -> Color {
        return self.cl.mul(self.ls);
    }

    fn shade(&self, hit: &RayHit) -> Color {
        let w = hit.hittable.normal(&hit.hit_point);
        let v = w.cross(&Vec3::new(0.0072, 1.0, 0.0034)).normalize();
        let u = v.cross(&w);

        let amount = get_hemisphere_sampler(self.sample_points_sqrt)
            .into_iter()
            .map(|sp| {
                let dir = u.mul(sp.x).add(v.mul(sp.y)).add(w.mul(sp.z)).normalize();
                if is_in_shadow(&hit.hit_point, &dir, &hit.models, true) {
                    return 0.0;
                } else {
                    return 1.0;
                }
            })
            .sum::<f64>()
            / (self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);

        return self.radiance().mul(amount);
    }
}

impl Light for PointLight {
    fn radiance(&self) -> Color {
        return self.cl.mul(self.ls);
    }

    fn shade(&self, _hit: &RayHit) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}

impl Light for DirectionalLight {
    fn radiance(&self) -> Color {
        return self.cl.mul(self.ls);
    }

    fn shade(&self, hit: &RayHit) -> Color {
        let l = self.direction.sub(&hit.hit_point).normalize();
        let kd = hit.material.diffuse_reflection;
        let cd = hit.material.diffuse_color;
        let n = hit.hittable.normal(&hit.hit_point);
        return cd
            .mul(kd)
            .mul(1.0 / 3.14)
            .mul(n.dot(&l).max(0.0))
            .mul(self.radiance());
    }
}

impl Light for AreaLight {
    fn radiance(&self) -> Color {
        return self.cl.mul(self.ls);
    }

    fn shade(&self, hit: &RayHit) -> Color {
        let w = hit.ray.origin.sub(hit.hit_point).normalize();
        let l = self.location.sub(hit.hit_point).normalize();
        let kd = hit.material.diffuse_reflection;
        let cd = hit.material.diffuse_color;
        let ks = hit.material.specular_refection;
        let e = hit.material.shininess;
        let n = hit.hittable.normal(&hit.hit_point);

        let mut shadow_intensity = 1.0;
        if n.dot(&w) > 0.0 {
            shadow_intensity = self.intensity_at(&hit.hit_point, &hit.models)
        }

        if shadow_intensity <= 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // diffuse
        let diffuse_amount = n.dot(&l).max(0.0);
        let diffuse = cd
            .mul(kd)
            .mul(1.0 / 3.14)
            .mul(diffuse_amount)
            .mul(self.radiance());

        // specular
        let r = n.mul(2.0 * diffuse_amount).sub(l);
        let specular_amount = r.dot(&w);
        let specular = self
            .cl
            .mul(ks * specular_amount.powf(e) * diffuse_amount * self.ls);
        return diffuse.add(specular).mul(shadow_intensity);
    }
}

impl AreaLight {
    fn radiance(&self) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }

    fn intensity_at(&self, point: &Vec3, models: &Vec<Model>) -> f64 {
        let x = self.location.x - self.width / 2.0;
        let z = self.location.z - self.height / 2.0;
        return get_unit_square_sampler(self.sample_points_sqrt)
            .map(|(dx, dz)| {
                let new_location =
                    Vec3::new(x + dx * self.width, self.location.y, z + dz * self.width);
                let dir = new_location.sub(point).normalize();
                if is_in_shadow(&point, &dir, &models, false) {
                    return 0.0;
                } else {
                    return 1.0;
                }
            })
            .sum::<f64>()
            / (self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }
}

fn is_in_shadow(point: &Vec3, dir: &Vec3, models: &Vec<Model>, test_object_only: bool) -> bool {
    let shadow_ray = Ray {
        origin: point.add(dir.mul(0.00001)),
        dir: *dir,
    };
    for m in models.iter() {
        if m.material.transparent {
            continue;
        }
        if test_object_only && !m.material.is_object {
            continue;
        }
        if m.aabb.intersects(&shadow_ray) {
            for h in m.hittables.iter() {
                if let Some(_) = h.intersects(&shadow_ray) {
                    return true;
                }
            }
        }
    }
    return false;
}
