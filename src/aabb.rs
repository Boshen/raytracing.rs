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
    pub fn intersects(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        let mut t1 = (self.min[0] - r.origin[0]) * r.inv_dir[0];
        let mut t2 = (self.max[0] - r.origin[0]) * r.inv_dir[0];

        let mut tmin = t1.min(t2).max(tmin);
        let mut tmax = t1.max(t2).min(tmax);

        for i in 1..3 {
            t1 = (self.min[i] - r.origin[i]) * r.inv_dir[i];
            t2 = (self.max[i] - r.origin[i]) * r.inv_dir[i];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmax >= tmin.max(0.0)
    }

    pub fn get_surrounding_aabb(box0: &AABB, box1: &AABB) -> AABB {
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
        AABB::new(small, big)
    }
}
