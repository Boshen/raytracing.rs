use std::collections::HashMap;
use std::sync::Arc;

use nalgebra::Point3;
use tobj::{load_obj, LoadOptions};

use crate::brdf::{GlossySpecular, Lambertian, PerfectSpecular};
use crate::color::Color;
use crate::geometric_object::{Geometry, Sphere, Triangle};
use crate::light::{AreaLight, LightEnum};
use crate::material::{Emissive, Material, Matte, Reflective};

pub struct Object {
    pub name: String,
    pub vertices: Vec<Point3<f64>>,
    pub face_indexes: Vec<(usize, usize, usize)>,
}

pub struct Asset {
    pub objects: Vec<Object>,
    pub geometries: Vec<Arc<Geometry>>,
    pub lights: Vec<LightEnum>,
    pub materials: HashMap<usize, Box<Material>>,
}

impl Asset {
    pub fn new(file_name: &str) -> Asset {
        let mut asset = Asset {
            objects: vec![],
            geometries: vec![],
            lights: vec![],
            materials: HashMap::new(),
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

            let mut triangles: Vec<Geometry> = vec![];

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
                    let material = if m.ambient[0] > 1.0 {
                        let emissive = Emissive::new(m.ambient[0] as f64, diffuse);
                        Material::Emissive(emissive)
                    } else {
                        let ambient_brdf = Lambertian::new(0.5, ambient);
                        let diffuse_brdf = Lambertian::new(1.0, diffuse);
                        Material::Matte(Matte::new(ambient_brdf, diffuse_brdf))
                    };

                    for f in 0..(mesh.indices.len() / 3) {
                        let start = f * 3;
                        let face_indices: Vec<_> = mesh.indices[start..start + 3].iter().collect();
                        let v1 = vertices[*face_indices[0] as usize];
                        let v2 = vertices[*face_indices[1] as usize];
                        let v3 = vertices[*face_indices[2] as usize];
                        let triangle = Triangle::new(material_id, v1, v2, v3, scale);
                        let triangle2 = Triangle::new(material_id, v1, v2, v3, scale);
                        triangles.push(Geometry::from(triangle2));
                        asset.geometries.push(Arc::new(Geometry::from(triangle)));
                    }

                    if m.ambient[0] > 1.0 {
                        let emissive = Emissive::new(m.ambient[0] as f64, diffuse);
                        let arealight = AreaLight::new(triangles, emissive);
                        asset.lights.push(LightEnum::from(arealight));
                    }

                    asset.materials.insert(material_id, Box::new(material));
                }
            };
        }

        let material = Material::Reflective(Reflective::new(
            Lambertian::new(0.1, Color::new(1.0, 1.0, 1.0)),
            Lambertian::new(0.1, Color::new(1.0, 1.0, 1.0)),
            GlossySpecular::new(0.2, 2.0),
            PerfectSpecular::new(0.5, Color::new(1.0, 1.0, 1.0)),
        ));
        let material_id = 1000_usize;
        asset.geometries.push(Arc::new(Geometry::from(Sphere::new(
            material_id,
            40.0,
            Point3::new(400.0, 40.0, 500.0),
            scale,
        ))));
        asset.materials.insert(material_id, Box::new(material));
        asset
    }
}
