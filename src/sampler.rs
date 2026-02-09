//! Sampling strategies for antialiasing and Monte Carlo integration.

use std::f64::consts::FRAC_PI_4;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use nalgebra::{Point2, Point3};
use num_integer::Roots;
use rand::{RngExt, SeedableRng, distr::StandardUniform, rngs::SmallRng};

use crate::model::Vec3;

fn make_rng() -> SmallRng {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let time_seed = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| {
        duration.as_secs().rotate_left(32) ^ u64::from(duration.subsec_nanos())
    });
    let counter_seed = COUNTER.fetch_add(0x9E37_79B9_7F4A_7C15, Ordering::Relaxed);
    SmallRng::seed_from_u64(time_seed ^ counter_seed)
}

/// Generates sample points for various rendering techniques including
/// antialiasing, area lighting, and Monte Carlo integration.
pub struct Sampler {
    /// Number of sample points per pixel
    num_samples: u8,
    /// Number of different sample sets to reduce correlation artifacts
    num_sets: usize,
    /// Pre-computed sample points in unit square
    samples: Vec<(f64, f64)>,
}

impl Sampler {
    /// Creates a new sampler with the specified number of samples per pixel.
    /// Uses jittered sampling for better antialiasing quality.
    #[must_use]
    pub fn new(num_samples: u8) -> Self {
        let num_sets = 83; // sufficiently large prime number to reduce correlation

        let n = num_samples.sqrt();
        let mut rng = make_rng();
        let mut samples = vec![];

        // Generate jittered samples for better quality than regular grid
        for _ in 0..num_sets {
            for j in 0..n {
                for k in 0..n {
                    let n = f64::from(n);
                    let (dx, dy) = if num_samples == 1 {
                        (0.5, 0.5) // Center sample for single sample
                    } else {
                        (
                            rng.sample::<f64, _>(StandardUniform),
                            rng.sample::<f64, _>(StandardUniform),
                        )
                    };
                    let point = ((f64::from(k) + dx) / n, (f64::from(j) + dy) / n);
                    samples.push(point);
                }
            }
        }

        Self { num_samples, num_sets, samples }
    }

    /// Returns the number of samples per pixel
    #[must_use]
    pub const fn count(&self) -> u8 {
        self.num_samples
    }

    /// Returns sample points in unit square [0,1)²
    fn unit_square(&self) -> Vec<(f64, f64)> {
        // Take a random set to avoid shading streaks
        let mut rng = make_rng();
        let skip: usize = rng.random_range(0..self.num_sets);

        self.samples.iter().skip(skip).take(self.count().into()).copied().collect()
    }

    /// Returns sample points in unit square as Point2 coordinates
    pub fn square(&self) -> impl Iterator<Item = Point2<f64>> {
        self.unit_square().into_iter().map(|(x, y)| Point2::new(x, y))
    }

    /// Maps unit square samples to barycentric coordinates on a triangle
    pub fn triangle<'a>(
        &'a self,
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

    /// Maps unit square samples to hemisphere using cosine-weighted distribution
    /// for importance sampling in global illumination
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

    /// Maps unit square samples to unit disk using concentric mapping
    /// for depth of field and area light sampling  
    pub fn disk(&self) -> impl Iterator<Item = (Point2<f64>, Point2<f64>)> {
        self.square().map(|p| {
            let x = 2.0f64.mul_add(p.x, -1.0);
            let y = 2.0f64.mul_add(p.y, -1.0);
            let (r, phi) = if x > -y {
                if x > y { (x, y / x) } else { (y, 2.0 - x / y) }
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
