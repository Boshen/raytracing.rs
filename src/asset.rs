use std::sync::Arc;

use nalgebra::Point3;
use tobj::{load_obj, LoadOptions};

use crate::{
    brdf::Lambertian,
    color::Color,
    geometric_object::{Geometry, Triangle},
    light::{Area, Light},
    material::{Emissive, Matte},
};

pub struct Object {
    pub name: String,
    pub vertices: Vec<Point3<f64>>,
    pub face_indexes: Vec<(usize, usize, usize)>,
}

pub struct Asset {
    pub objects: Vec<Object>,
    pub geometries: Vec<Arc<dyn Geometry>>,
    pub lights: Vec<Arc<dyn Light>>,
}

impl Asset {
    /// # Panics
    #[must_use]
    pub fn new(file_name: &str, scale: f64) -> Self {
        let mut asset = Self { objects: vec![], geometries: vec![], lights: vec![] };

        let (models, materials) =
            load_obj(file_name, &LoadOptions { triangulate: true, ..LoadOptions::default() })
                .expect("Failed to load file");

        let materials = materials.expect("loaded materials");

        for model in &models {
            let mesh = &model.mesh;
            let mut vertices: Vec<Point3<f64>> = vec![];
            for v in 0..mesh.positions.len() / 3 {
                vertices.push(Point3::new(
                    f64::from(mesh.positions[3 * v]),
                    f64::from(mesh.positions[3 * v + 1]),
                    f64::from(mesh.positions[3 * v + 2]),
                ));
            }

            let mut triangles: Vec<Arc<dyn Geometry>> = vec![];

            match mesh.material_id {
                None => {}
                Some(material_id) => {
                    let m = &materials[material_id];
                    let ambient = Color::new(
                        f64::from(m.ambient[0]),
                        f64::from(m.ambient[1]),
                        f64::from(m.ambient[2]),
                    );

                    let diffuse = Color::new(
                        f64::from(m.diffuse[0]),
                        f64::from(m.diffuse[1]),
                        f64::from(m.diffuse[2]),
                    );

                    for f in 0..(mesh.indices.len() / 3) {
                        let start = f * 3;
                        let face_indices: Vec<_> = mesh.indices[start..start + 3].iter().collect();
                        let v1 = vertices[*face_indices[0] as usize];
                        let v2 = vertices[*face_indices[1] as usize];
                        let v3 = vertices[*face_indices[2] as usize];

                        let triangle: Arc<dyn Geometry> = if m.ambient[0] > 1.0 {
                            let material = Emissive::new(f64::from(m.ambient[0]), diffuse);
                            Arc::new(Triangle::new(material, v1, v2, v3, scale))
                        } else {
                            let ambient_brdf = Lambertian::new(0.5, ambient);
                            let diffuse_brdf = Lambertian::new(1.0, diffuse);
                            let material = Matte::new(ambient_brdf, diffuse_brdf);
                            Arc::new(Triangle::new(material, v1, v2, v3, scale))
                        };

                        triangles.push(triangle);
                    }

                    if m.ambient[0] > 1.0 {
                        let emissive = Emissive::new(f64::from(m.ambient[0]), diffuse);
                        let arealight = Arc::new(Area::new(triangles.clone(), emissive));
                        asset.lights.push(arealight);
                    }

                    asset.geometries.extend(triangles);
                }
            };
        }

        asset
    }
}
