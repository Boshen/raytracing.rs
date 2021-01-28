use crate::model::Vec3;
use crate::ray::Ray;

pub struct AABB {
    min: [f64; 3],
    max: [f64; 3],
}

impl AABB {
    pub fn new(mins: Vec<Vec3>, maxs: Vec<Vec3>) -> AABB {
        return AABB {
            min: [
                f(&mins, &|v| v.x, &|a, b| a.min(b)),
                f(&mins, &|v| v.y, &|a, b| a.min(b)),
                f(&mins, &|v| v.z, &|a, b| a.min(b)),
            ],
            max: [
                f(&maxs, &|v| v.x, &|a, b| a.max(b)),
                f(&maxs, &|v| v.y, &|a, b| a.max(b)),
                f(&maxs, &|v| v.z, &|a, b| a.max(b)),
            ],
        };
    }

    // https://tavianator.com/2015/ray_box_nan.html
    pub fn intersects(&self, ray: &Ray) -> bool {
        let origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let inv_dir = [1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z];

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

        return tmax >= tmin.max(0.0);
    }
}

fn f(xs: &Vec<Vec3>, acc: &dyn Fn(&Vec3) -> f64, map: &dyn Fn(f64, f64) -> f64) -> f64 {
    return xs
        .iter()
        .map(|x| acc(&x))
        .fold(f64::INFINITY, |a, b| map(a, b));
}
