use rand::Rng;

pub const SAMPLE_POINTS: u32 = 5;

pub fn get_unit_square_sampler() -> Vec<(f64, f64)> {
    let n = SAMPLE_POINTS;
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
