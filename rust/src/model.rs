use nalgebra::Vector3;

use crate::aabb::AABB;
use crate::hittable::Hittable;

pub type Color = Vector3<f64>;
pub type Vec3 = Vector3<f64>;

pub struct Model {
    pub material: Material,
    pub hittables: Vec<Box<dyn Hittable>>,
    pub aabb: AABB,
}

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_reflection: f64,
    pub diffuse_color: Color,
    pub reflection: f64,
    pub specular_refection: f64,
    pub shininess: f64,
    pub transparent: bool,
    pub is_object: bool,
}

impl Model {
    pub fn new(l: f64, material: Material, hittables: Vec<Box<dyn Hittable>>) -> Model {
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
        let mins = self.hittables.iter().map(|h| h.get_min_point()).collect();
        let maxs = self.hittables.iter().map(|h| h.get_max_point()).collect();
        self.aabb = AABB::new(mins, maxs)
    }
}
