use nalgebra::Vector3;

use crate::aabb::AABB;
use crate::geometric_object::{GeometricObject, Geometry};
use crate::material::Material;
use crate::ray::Ray;

pub type Vec3 = Vector3<f64>;

pub struct Model {
    pub name: String,
    pub material: Box<Material>,
    pub geometric_objects: Vec<Geometry>,
    pub aabb: AABB,
}

impl Model {
    pub fn new(name: String, material: Box<Material>, geometric_objects: Vec<Geometry>) -> Model {
        let (mins, maxs) = geometric_objects
            .iter()
            .map(|h| (h.get_min_point(), h.get_max_point()))
            .unzip();
        Model {
            name,
            material,
            geometric_objects,
            aabb: AABB::new(mins, maxs),
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<(f64, Geometry)> {
        if !self.aabb.intersects(&ray) {
            return None;
        }
        self.geometric_objects
            .iter()
            .filter_map(|o| o.intersects(ray).map(|dist| (dist, o.clone())))
            .min_by(|t1, t2| (t1.0).partial_cmp(&t2.0).unwrap())
    }
}
