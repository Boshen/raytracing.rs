use nalgebra::Point3;
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::geometric_object::Geometry;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};

pub struct BvhNode {
    pub left: Arc<dyn Geometry>,
    pub right: Arc<dyn Geometry>,
    pub aabb: Aabb,
    pub children: usize,
}

impl BvhNode {
    pub fn new(objects: Vec<Arc<dyn Geometry>>, start: usize, end: usize) -> Self {
        let mut objects = objects;
        let axis = thread_rng().gen_range(0..3);
        let comparator = box_compare(axis);

        let span = end - start;
        if span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                aabb: objects[start].get_bounding_box(),
                children: 1,
            }
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + span / 2;
            let left = Arc::new(Self::new(objects.clone(), start, mid));
            let right = Arc::new(Self::new(objects, mid, end));
            let box_left = left.get_bounding_box();
            let box_right = right.get_bounding_box();
            Self {
                left,
                right,
                aabb: Aabb::get_surrounding_aabb(&box_left, &box_right),
                children: 2,
            }
        }
    }
}

impl Geometry for BvhNode {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.intersects(ray, t_min, t_max) {
            return None;
        }
        if self.children == 1 {
            return self.left.intersects(ray, t_min, t_max);
        }
        self.left.intersects(ray, t_min, t_max).map_or_else(
            || self.right.intersects(ray, t_min, t_max),
            |r1| self.right.intersects(ray, t_min, r1.dist).or(Some(r1)),
        )
    }

    fn scale(&mut self, _l: f64) {}

    fn normal(&self, _p: &Point3<f64>) -> Vec3 {
        Vec3::zeros()
    }

    fn get_center(&self) -> Point3<f64> {
        Point3::origin()
    }

    fn get_min_point(&self) -> Point3<f64> {
        self.aabb.min
    }

    fn get_max_point(&self) -> Point3<f64> {
        self.aabb.max
    }

    fn get_bounding_box(&self) -> Aabb {
        Aabb::new(self.get_min_point(), self.get_max_point())
    }

    fn get_samples(&self, _sample_points_sqrt: u8) -> Vec<Point3<f64>> {
        vec![]
    }
}

fn box_compare<T: ?Sized>(axis: usize) -> Box<dyn Fn(&Arc<T>, &Arc<T>) -> std::cmp::Ordering>
where
    T: Geometry,
{
    Box::new(move |a, b| {
        let box_a = a.get_bounding_box();
        let box_b = b.get_bounding_box();
        box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
    })
}
