use std::sync::Arc;

use nalgebra::Point3;
use tobj::{load_obj, LoadOptions};

use crate::brdf::{GlossySpecular, Lambertian, PerfectSpecular};
use crate::color::Color;
use crate::geometric_object::{Geometry, Sphere, Triangle};
use crate::light::{Area, Light};
use crate::material::{Emissive, Matte, Reflective};

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
    #[must_use]
    pub fn new(file_name: &str) -> Self {
        let mut asset = Self {
            objects: vec![],
            geometries: vec![],
            lights: vec![],
        };

        let (models, materials) = load_obj(
            &file_name,
            &LoadOptions {
                triangulate: true,
                ..LoadOptions::default()
            },
        )
        .expect("Failed to load file");

        let materials = materials.expect("loaded materials");
        let scale = 555.0;

        for model in &models {
            let mesh = &model.mesh;
            let mut vertices: Vec<Point3<f64>> = vec![];
            for v in 0..mesh.positions.len() / 3 {
                vertices.push(Point3::new(
                    mesh.positions[3 * v] as f64,
                    mesh.positions[3 * v + 1] as f64,
                    mesh.positions[3 * v + 2] as f64,
                ));
            }

            let mut triangles: Vec<Arc<dyn Geometry>> = vec![];

            match mesh.material_id {
                None => {}
                Some(material_id) => {
                    let m = &materials[material_id];
                    let ambient = Color::new(
                        m.ambient[0] as f64,
                        m.ambient[1] as f64,
                        m.ambient[2] as f64,
                    );

                    let diffuse = Color::new(
                        m.diffuse[0] as f64,
                        m.diffuse[1] as f64,
                        m.diffuse[2] as f64,
                    );

                    for f in 0..(mesh.indices.len() / 3) {
                        let start = f * 3;
                        let face_indices: Vec<_> = mesh.indices[start..start + 3].iter().collect();
                        let v1 = vertices[*face_indices[0] as usize];
                        let v2 = vertices[*face_indices[1] as usize];
                        let v3 = vertices[*face_indices[2] as usize];

                        let triangle: Arc<dyn Geometry> = if m.ambient[0] > 1.0 {
                            let material = Emissive::new(m.ambient[0] as f64, diffuse);
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
                        let emissive = Emissive::new(m.ambient[0] as f64, diffuse);
                        let arealight = Arc::new(Area::new(triangles.clone(), emissive));
                        asset.lights.push(arealight);
                    }

                    asset.geometries.extend(triangles);
                }
            };
        }

        let material = Reflective::new(
            Lambertian::new(0.1, Color::new(1.0, 1.0, 1.0)),
            Lambertian::new(0.1, Color::new(1.0, 1.0, 1.0)),
            GlossySpecular::new(0.2, 2.0),
            PerfectSpecular::new(0.5, Color::new(1.0, 1.0, 1.0)),
        );
        asset.geometries.push(Arc::new(Sphere::new(
            material,
            40.0,
            Point3::new(400.0, 40.0, 500.0),
            scale,
        )));
        asset
    }
}