use rand::Rng;

use crate::model::Vec3;

pub fn get_unit_square_sampler(n: u32) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    return (0..n)
        .into_iter()
        .flat_map(|i| {
            let dx = (i as f64 + rng.gen_range(0.0, 1.0)) / n as f64;
            return (0..n).into_iter().map(move |j| {
                let dy = (j as f64 + rng.gen_range(0.0, 1.0)) / n as f64;
                return (dx, dy);
            });
        })
        .collect();
}

pub fn get_hemisphere_sampler(n: u32) -> Vec<Vec3> {
    let e = 1.0;
    return get_unit_square_sampler(n)
        .into_iter()
        .map(|(x, y)| {
            let phi = 2.0 * std::f64::consts::PI * x;
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();
            let cos_theta = (1.0 - y).powf(1.0 / (e + 1.0));
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            return Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
        })
        .collect();
}
