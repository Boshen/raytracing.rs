use crate::ray::Ray;
use nalgebra::Point3;

pub struct AABB {
    min: Point3<f64>,
    max: Point3<f64>,
}

impl AABB {
    pub fn new(min: Point3<f64>, max: Point3<f64>) -> AABB {
        AABB { min, max }
    }

    // https://tavianator.com/2015/ray_box_nan.html
    pub fn intersects(&self, ray: &Ray) -> bool {
        let origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let inv_dir = [ray.dir.x.recip(), ray.dir.y.recip(), ray.dir.z.recip()];

        let mut t1 = (self.min[0] - origin[0]) * inv_dir[0];
        let mut t2 = (self.max[0] - origin[0]) * inv_dir[0];

        let mut tmin = t1.min(t2);
        let mut tmax = t1.max(t2);

        for i in 1..3 {
            t1 = (self.min[i] - origin[i]) * inv_dir[i];
            t2 = (self.max[i] - origin[i]) * inv_dir[i];
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmax >= tmin.max(0.0)
    }
}
