use nalgebra::Dot;
use std::ops::{Div, Mul, Sub};

use crate::model::{Color, Vec3};
use crate::ray::RayHit;

pub trait BRDF: Send + Sync {
    // reciprocity
    fn f(&self, hit: &RayHit, wo: &Vec3, wi: &Vec3) -> Color;
    // bihemispherical reflectance
    fn rho(&self) -> Color;
    fn sample_f(&self, hit: &RayHit, wo: &Vec3, wi: &Vec3) -> Color;
}

pub struct Lambertian {
    pub kd: f64,   // diffuse reflection coefficient [0, 1]
    pub cd: Color, // diffuse color
}

pub struct PerfectSpecular {
    pub kr: f64,   // reflection coefficient
    pub cr: Color, // reflection color
}

pub struct GlossySpecular {
    pub ks: f64,  // specular reflection coefficient
    pub exp: f64, // shininess
}

impl BRDF for Lambertian {
    fn f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        let inv_pi = 1.0 / std::f64::consts::PI;
        return self.rho().mul(inv_pi);
    }

    fn rho(&self) -> Color {
        return self.cd.mul(self.kd);
    }

    fn sample_f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}

impl BRDF for PerfectSpecular {
    fn f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        return Color::new(0.0, 0.0, 0.0); // is black for PerfectSpecular
    }

    fn rho(&self) -> Color {
        return Color::new(0.0, 0.0, 0.0); // is black for PerfectSpecular
    }

    fn sample_f(&self, hit: &RayHit, _wo: &Vec3, wi: &Vec3) -> Color {
        let normal = hit.normal();
        return self.cr.mul(self.kr).div(normal.dot(&wi));
    }
}

impl BRDF for GlossySpecular {
    fn f(&self, hit: &RayHit, wo: &Vec3, wi: &Vec3) -> Color {
        let ndotwi = hit.normal().dot(wi);
        let r = hit.normal().mul(ndotwi).mul(2.0).sub(wi);
        let rdotwo = r.dot(wo);
        if rdotwo <= 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let s = self.ks * rdotwo.powf(self.exp);
        return Color::new(s, s, s);
    }

    fn rho(&self) -> Color {
        return Color::new(0.0, 0.0, 0.0); // is black for PerfectSpecular
    }

    fn sample_f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}
