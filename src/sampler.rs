use nalgebra::{Point2, Point3};
use num_integer::Roots;
use rand::{distributions::Standard, thread_rng, Rng};
use std::f64::consts::FRAC_PI_4;

use crate::model::Vec3;

pub struct Sampler {
    /// number of sample points
    num_samples: u8,

    /// number of sample sets
    num_sets: usize,

    /// sets of samples
    samples: Vec<(f64, f64)>,
}

impl Sampler {
    #[must_use]
    pub fn new(num_samples: u8) -> Self {
        let num_sets = 83; // suffieciently large prime number

        let n = num_samples.sqrt();
        let mut rng = thread_rng();
        let mut samples = vec![];

        // jitted samples
        for _ in 0..num_sets {
            for j in 0..n {
                for k in 0..n {
                    let n = f64::from(n);
                    let (dx, dy) = if num_samples == 1 {
                        (0.0, 0.0)
                    } else {
                        (
                            rng.sample::<f64, _>(Standard),
                            rng.sample::<f64, _>(Standard),
                        )
                    };
                    let point = ((f64::from(k) + dx) / n, (f64::from(j) + dy) / n);
                    samples.push(point);
                }
            }
        }

        Self {
            num_samples,
            num_sets,
            samples,
        }
    }

    #[must_use]
    pub const fn count(&self) -> u8 {
        self.num_samples
    }

    fn unit_square(&self) -> Vec<(f64, f64)> {
        // take a random set to avoid shading streaks
        let mut rng = thread_rng();
        let skip: usize = rng.gen_range(0..self.num_sets);

        self.samples
            .iter()
            .skip(skip)
            .take(self.count().into())
            .copied()
            .collect()
    }

    pub fn square(&self) -> impl Iterator<Item = Point2<f64>> {
        self.unit_square()
            .into_iter()
            .map(|(x, y)| Point2::new(x, y))
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
            let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();
            Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
        })
    }

    pub fn disk(&self) -> impl Iterator<Item = (Point2<f64>, Point2<f64>)> {
        self.square().map(|p| {
            let x = 2.0f64.mul_add(p.x, -1.0);
            let y = 2.0f64.mul_add(p.y, -1.0);
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
