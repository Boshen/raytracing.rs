use crate::ray::Ray;
use nalgebra::Point3;

#[derive(Copy, Clone)]
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
        let origin = [r.origin.x, r.origin.y, r.origin.z];
        let inv_dir = [r.dir.x.recip(), r.dir.y.recip(), r.dir.z.recip()];

        let mut t1 = (self.min[0] - origin[0]) * inv_dir[0];
        let mut t2 = (self.max[0] - origin[0]) * inv_dir[0];

        let mut tmin = t1.min(t2).max(t_min);
        let mut tmax = t1.max(t2).min(t_max);

        for i in 1..3 {
            t1 = (self.min[i] - origin[i]) * inv_dir[i];
            t2 = (self.max[i] - origin[i]) * inv_dir[i];
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmax >= tmin.max(0.0)
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
