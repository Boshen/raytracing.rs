use crate::ray::Ray;
use nalgebra::Point3;

pub struct Aabb {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

impl Aabb {
    #[must_use]
    pub const fn new(min: Point3<f64>, max: Point3<f64>) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn intersects(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for i in 0..3 {
            let t1 = (self.min[i] - r.origin[i]) / r.dir[i];
            let t2 = (self.max[i] - r.origin[i]) / r.dir[i];
            tmin = t1.min(t2).max(tmin);
            tmax = t1.max(t2).min(tmax);
            if tmax < tmin {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn get_surrounding_aabb(box0: &Self, box1: &Self) -> Self {
        let small = Point3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Point3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        Self::new(small, big)
    }
}
