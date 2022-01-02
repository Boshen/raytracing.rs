use nalgebra::{Point2, Point3};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::f64::consts::FRAC_PI_4;

use crate::model::Vec3;

pub struct Sampler {
    pub n: u8, // number of sample points
}

impl Sampler {
    #[must_use]
    pub const fn new(sample_points: u8) -> Self {
        Self { n: sample_points }
    }

    pub fn square(&self) -> impl Iterator<Item = Point2<f64>> {
        thread_rng()
            .sample_iter(&Standard)
            .take(self.n.into())
            .map(|(i, j)| Point2::new(i, j))
    }

    pub fn triangle<'a>(
        &self,
        x: &'a Point3<f64>,
        y: &'a Point3<f64>,
        z: &'a Point3<f64>,
    ) -> impl Iterator<Item = Point3<f64>> + 'a {
        self.square().map(move |p| {
            let mut a = p.x;
            let mut b = p.y;
            if a + b >= 1.0 {
                a = 1.0 - a;
                b = 1.0 - b;
            }
            x + ((y - x) * a) + ((z - x) * b)
        })
    }

    pub fn hemisphere(&self) -> impl Iterator<Item = Vec3> {
        self.square().map(|p| {
            let e = 1.0;
            let phi = 2.0 * std::f64::consts::PI * p.x;
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();
            let cos_theta = (1.0 - p.y).powf((e + 1.0_f64).recip());
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
        })
    }

    pub fn disk(&self) -> impl Iterator<Item = (Point2<f64>, Point2<f64>)> {
        self.square().map(|p| {
            let x = 2.0 * p.x - 1.0;
            let y = 2.0 * p.y - 1.0;
            let (r, phi) = if x > -y {
                if x > y {
                    (x, y / x)
                } else {
                    (y, 2.0 - x / y)
                }
            } else if x < y {
                (-x, 4.0 + y / x)
            } else {
                (-y, if y == 0.0 { 0.0 } else { 6.0 - x / y })
            };
            let phi_ = phi * FRAC_PI_4;
            (p, Point2::new(r * phi_.cos(), r * phi_.sin()))
        })
    }
}
