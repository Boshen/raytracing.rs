use nalgebra::Dot;
use num_traits::identities::Zero;
use std::ops::{Div, Mul, Sub};

use crate::color::Color;
use crate::model::Vec3;
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

pub struct GlossySpecular {
    pub ks: f64,  // specular reflection coefficient [0, 1]
    pub exp: f64, // shininess
}

pub struct PerfectSpecular {
    pub kr: f64,   // reflection coefficient
    pub cr: Color, // reflection color
}

impl Lambertian {
    pub fn new(kd: f64, cd: Color) -> Lambertian {
        Lambertian { kd, cd }
    }
}

impl PerfectSpecular {
    pub fn new(kr: f64, cr: Color) -> PerfectSpecular {
        PerfectSpecular { kr, cr }
    }
}

impl GlossySpecular {
    pub fn new(ks: f64, exp: f64) -> GlossySpecular {
        GlossySpecular { ks, exp }
    }
}

impl BRDF for Lambertian {
    fn f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        self.rho().div(std::f64::consts::PI)
    }

    fn rho(&self) -> Color {
        self.cd.mul(self.kd)
    }

    fn sample_f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zero()
    }
}

impl BRDF for GlossySpecular {
    fn f(&self, hit: &RayHit, wo: &Vec3, wi: &Vec3) -> Color {
        let ndotwi = hit.normal.dot(wi).max(0.0);
        let r = hit.normal.mul(2.0 * ndotwi).sub(wi);
        let rdotwo = r.dot(wo);
        if rdotwo <= 0.0 {
            return Color::zero();
        }
        let s = self.ks * rdotwo.powf(self.exp);
        Color::new(s, s, s)
    }

    fn rho(&self) -> Color {
        Color::zero() // is black for GlossySpecular
    }

    fn sample_f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zero()
    }
}

impl BRDF for PerfectSpecular {
    fn f(&self, _hit: &RayHit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zero() // is black for PerfectSpecular
    }

    fn rho(&self) -> Color {
        Color::zero() // is black for PerfectSpecular
    }

    fn sample_f(&self, hit: &RayHit, _wo: &Vec3, wi: &Vec3) -> Color {
        self.cr.mul(self.kr).div(hit.normal.dot(&wi))
    }
}
