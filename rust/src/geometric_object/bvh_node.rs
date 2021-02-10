use nalgebra::Point3;
use num_traits::identities::Zero;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

use crate::aabb::AABB;
use crate::geometric_object::GeometricObject;
use crate::geometric_object::Geometry;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};

#[derive(Clone)]
pub struct BvhNode {
    pub left: Box<Geometry>,
    pub right: Box<Geometry>,
    pub aabb: AABB,
}

impl GeometricObject for BvhNode {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.intersects(ray, t_min, t_max) {
            return None;
        }
        self.left.intersects(ray, t_min, t_max).map_or_else(
            || self.right.intersects(ray, t_min, t_max),
            |r1| self.right.intersects(ray, t_min, r1.dist).or(Some(r1)),
        )
    }

    fn scale(&mut self, _l: f64) {}

    fn normal(&self, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }

    fn get_center(&self) -> Vec3 {
        Vec3::zero()
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

    fn get_samples(&self, _sample_points_sqrt: usize) -> Vec<Vec3> {
        vec![]
    }

    fn get_material_id(&self) -> usize {
        0
    }
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Geometry>, start: usize, end: usize) -> BvhNode {
        let axis = thread_rng().gen_range(0, 3);
        let comparator = box_compare(axis);

        let span = end - start;
        let (left, right) = match span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + span / 2.0 as usize;
                let left = BvhNode::new(objects, start, mid);
                let right = BvhNode::new(objects, mid, end);
                (Geometry::from(left), Geometry::from(right))
            }
        };

        let box_left = left.get_bounding_box();
        let box_right = right.get_bounding_box();

        BvhNode {
            left: Box::new(left),
            right: Box::new(right),
            aabb: AABB::get_surrounding_aabb(&box_left, &box_right),
        }
    }
}

fn box_compare(axis: usize) -> Box<dyn Fn(&Geometry, &Geometry) -> std::cmp::Ordering> {
    Box::new(move |a, b| {
        let box_a = a.get_bounding_box();
        let box_b = b.get_bounding_box();
        box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
    })
}
