use nalgebra::Vector3;

use crate::aabb::AABB;
use crate::hittable::Hittable;
use crate::material::Material;

pub type Vec3 = Vector3<f64>;

pub struct Model {
    pub material: Box<Material>,
    pub hittables: Vec<Box<dyn Hittable>>,
    pub aabb: AABB,
}

impl Model {
    pub fn new(material: Box<Material>, hittables: Vec<Box<dyn Hittable>>) -> Model {
        let (mins, maxs) = hittables
            .iter()
            .map(|h| (h.get_min_point(), h.get_max_point()))
            .unzip();
        return Model {
            material: material,
            hittables: hittables,
            aabb: AABB::new(mins, maxs),
        };
    }
}
