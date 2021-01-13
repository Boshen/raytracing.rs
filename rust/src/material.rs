use crate::model::Color;

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
