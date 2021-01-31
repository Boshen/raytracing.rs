use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::ops::{Add, Mul, Sub};

use crate::geometric_object::Triangle;
use crate::model::Vec3;

pub fn get_unit_square_sampler(n: u32) -> impl Iterator<Item = (f64, f64)> {
    thread_rng()
        .sample_iter(&Standard)
        .take(n as usize * n as usize)
}

pub fn get_triangle_sampler(n: u32, t: &Triangle) -> impl Iterator<Item = Vec3> {
    let x = t.0;
    let y = t.1;
    let z = t.2;
    get_unit_square_sampler(n).map(move |(mut a, mut b)| {
        if a + b >= 1.0 {
            a = 1.0 - a;
            b = 1.0 - b;
        }
        x.add(y.sub(x).mul(a)).add(z.sub(x).mul(b))
    })
}

pub fn get_hemisphere_sampler(n: u32) -> impl Iterator<Item = Vec3> {
    get_unit_square_sampler(n).map(|(x, y)| {
        let e = 1.0;
        let phi = 2.0 * std::f64::consts::PI * x;
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let cos_theta = (1.0 - y).powf((e + 1.0 as f64).recip());
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
    })
}
