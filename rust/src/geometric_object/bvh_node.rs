use nalgebra::Point3;
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::aabb::AABB;
use crate::geometric_object::GeometricObject;
use crate::geometric_object::Geometry;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};

#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<Geometry>,
    pub right: Arc<Geometry>,
    pub aabb: AABB,
    pub children: usize,
}

impl GeometricObject for BvhNode {
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

    fn get_bounding_box(&self) -> AABB {
        AABB::new(self.get_min_point(), self.get_max_point())
    }

    fn get_samples(&self, _sample_points_sqrt: usize) -> Vec<Point3<f64>> {
        vec![]
    }

    fn get_material_id(&self) -> usize {
        0
    }
}

impl BvhNode {
    pub fn new(objects: Vec<Arc<Geometry>>, start: usize, end: usize) -> BvhNode {
        let mut objects = objects;
        let axis = thread_rng().gen_range(0..3);
        let comparator = box_compare(axis);

        let span = end - start;
        if span == 1 {
            BvhNode {
                left: objects[start].clone(),
                right: objects[start].clone(),
                aabb: objects[start].get_bounding_box(),
                children: 1,
            }
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + span / 2.0 as usize;
            let left = Geometry::from(BvhNode::new(objects.clone(), start, mid));
            let right = Geometry::from(BvhNode::new(objects, mid, end));
            let box_left = left.get_bounding_box();
            let box_right = right.get_bounding_box();
            BvhNode {
                left: Arc::new(left),
                right: Arc::new(right),
                aabb: AABB::get_surrounding_aabb(&box_left, &box_right),
                children: 2,
            }
        }
    }
}

fn box_compare(axis: usize) -> Box<dyn Fn(&Arc<Geometry>, &Arc<Geometry>) -> std::cmp::Ordering> {
    Box::new(move |a, b| {
        let box_a = a.get_bounding_box();
        let box_b = b.get_bounding_box();
        box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
    })
}
