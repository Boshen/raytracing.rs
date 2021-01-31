use nalgebra::Vector3;

use crate::aabb::AABB;
use crate::geometric_object::GeometricObject;
use crate::material::Material;

pub type Vec3 = Vector3<f64>;

pub struct Model {
    pub name: String,
    pub material: Box<Material>,
    pub geometric_objects: Vec<Box<dyn GeometricObject>>,
    pub aabb: AABB,
}

impl Model {
    pub fn new(name: String, material: Box<Material>, geometric_objects: Vec<Box<dyn GeometricObject>>) -> Model {
        let (mins, maxs) = geometric_objects
            .iter()
            .map(|h| (h.get_min_point(), h.get_max_point()))
            .unzip();
        return Model {
            name,
            material,
            geometric_objects,
            aabb: AABB::new(mins, maxs),
        };
    }
}
