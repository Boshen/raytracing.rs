use std::sync::atomic::{AtomicU64, Ordering};

/// Number of rays created
pub static RAY_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn inc_ray_count() {
    RAY_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Number of intersections on geometries
pub static INTERSECTION_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn inc_intersection_count() {
    INTERSECTION_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn print_count() {
    println!("Number of Rays: {}", RAY_COUNT.load(Ordering::Relaxed),);
    println!(
        "Number of Intersections: {}",
        INTERSECTION_COUNT.load(Ordering::Relaxed),
    );
}
