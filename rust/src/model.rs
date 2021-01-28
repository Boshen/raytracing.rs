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
    pub fn new(l: f64, material: Box<Material>, hittables: Vec<Box<dyn Hittable>>) -> Model {
        let mut model = Model {
            material: material,
            hittables: hittables,
            aabb: AABB::new(vec![], vec![]),
        };
        model.scale(l);
        return model;
    }

    fn scale(&mut self, l: f64) {
        for h in &mut self.hittables {
            h.scale(l);
        }
        let (mins, maxs) = self
            .hittables
            .iter()
            .map(|h| (h.get_min_point(), h.get_max_point()))
            .unzip();
        self.aabb = AABB::new(mins, maxs)
    }
}
