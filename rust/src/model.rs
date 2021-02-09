use nalgebra::Vector3;

use crate::geometric_object::{GeometricObject, Geometry};
use crate::material::Material;
use crate::ray::Ray;

pub type Vec3 = Vector3<f64>;

pub struct Model {
    pub name: String,
    pub material: Box<Material>,
    pub geometric_objects: Vec<Geometry>,
}

impl Model {
    pub fn new(name: String, material: Box<Material>, geometric_objects: Vec<Geometry>) -> Model {
        Model {
            name,
            material,
            geometric_objects,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<(f64, Geometry)> {
        self.geometric_objects
            .iter()
            .filter(|o| o.get_bounding_box().intersects(&ray))
            .filter_map(|o| o.intersects(ray).map(|dist| (dist, *o)))
            .min_by(|t1, t2| (t1.0).partial_cmp(&t2.0).unwrap())
    }
}
