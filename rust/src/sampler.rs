use nalgebra::{Point2, Point3};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::f64::consts::FRAC_PI_4;

use crate::geometric_object::Triangle;
use crate::model::Vec3;

pub fn get_square_sampler(n: usize) -> impl Iterator<Item = Point2<f64>> {
    thread_rng()
        .sample_iter(&Standard)
        .take(n * n)
        .map(|(i, j)| Point2::new(i, j))
}

pub fn get_triangle_sampler(n: usize, t: &Triangle) -> impl Iterator<Item = Point3<f64>> {
    let (x, y, z) = (t.x, t.y, t.z);
    get_square_sampler(n).map(move |p| {
        let mut a = p.x;
        let mut b = p.y;
        if a + b >= 1.0 {
            a = 1.0 - a;
            b = 1.0 - b;
        }
        x + ((y - x) * a) + ((z - x) * b)
    })
}

pub fn get_hemisphere_sampler(n: usize) -> impl Iterator<Item = Vec3> {
    get_square_sampler(n).map(|p| {
        let e = 1.0;
        let phi = 2.0 * std::f64::consts::PI * p.x;
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let cos_theta = (1.0 - p.y).powf((e + 1.0_f64).recip());
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
    })
}

pub fn get_disk_sampler(n: usize) -> impl Iterator<Item = (Point2<f64>, Point2<f64>)> {
    get_square_sampler(n).map(|p| {
        let spx = 2.0 * p.x - 1.0;
        let spy = 2.0 * p.y - 1.0;
        let (r, phi) = if spx > -spy {
            if spx > spy {
                (spx, spy / spx)
            } else {
                (spy, 2.0 - spx / spy)
            }
        } else if spx < spy {
            (-spx, 4.0 + spy / spx)
        } else {
            (-spy, if spy == 0.0 { 0.0 } else { 6.0 - spx / spy })
        };
        let phi_ = phi * FRAC_PI_4;
        (p, Point2::new(r * phi_.cos(), r * phi_.sin()))
    })
}
