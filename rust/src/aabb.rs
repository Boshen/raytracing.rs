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
                get_min(&mins, &|v: &Vec3| v.x),
                get_min(&mins, &|v: &Vec3| v.y),
                get_min(&mins, &|v: &Vec3| v.z),
            ],
            max: [
                get_max(&maxs, &|v: &Vec3| v.x),
                get_max(&maxs, &|v: &Vec3| v.y),
                get_max(&maxs, &|v: &Vec3| v.z),
            ],
        };
    }

    // https://tavianator.com/2015/ray_box_nan.html
    pub fn intersects(&self, ray: &Ray) -> bool {
        let origin = [ray.start.x, ray.start.y, ray.start.z];
        let inv_dir = [
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        ];

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

fn get_min(xs: &Vec<Vec3>, acc: &dyn Fn(&Vec3) -> f64) -> f64 {
    return xs
        .iter()
        .map(|x| acc(&x))
        .fold(f64::INFINITY, |a, b| a.min(b));
}

fn get_max(xs: &Vec<Vec3>, acc: &dyn Fn(&Vec3) -> f64) -> f64 {
    return xs
        .iter()
        .map(|x| acc(&x))
        .fold(-f64::INFINITY, |a, b| a.max(b));
}
