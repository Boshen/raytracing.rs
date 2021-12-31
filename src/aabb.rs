use crate::ray::Ray;
use nalgebra::Point3;

pub struct AABB {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

impl AABB {
    pub fn new(min: Point3<f64>, max: Point3<f64>) -> AABB {
        AABB { min, max }
    }

    // https://tavianator.com/2015/ray_box_nan.html
    pub fn intersects(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;
        for i in 0..3 {
            let inv_d = r.dir[i].recip();
            let t1 = (self.min[i] - r.origin[i]) * inv_d;
            let t2 = (self.max[i] - r.origin[i]) * inv_d;
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
            if tmax < tmin.max(0.0) {
                return false;
            }
        }
        true
    }

    pub fn get_surrounding_aabb(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f64::min(box0.min.x, box1.min.x),
            f64::min(box0.min.y, box1.min.y),
            f64::min(box0.min.z, box1.min.z),
        );
        let big = Point3::new(
            f64::max(box0.max.x, box1.max.x),
            f64::max(box0.max.y, box1.max.y),
            f64::max(box0.max.z, box1.max.z),
        );
        AABB::new(small, big)
    }
}
