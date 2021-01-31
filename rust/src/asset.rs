extern crate tobj;

use crate::brdf::Lambertian;
use crate::color::Color;
use crate::geometric_object::{GeometricObject, Triangle};
use crate::light::{AreaLight, Light};
use crate::material::{Emissive, Material, Matte};
use crate::model::{Model, Vec3};

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub vertices: Vec<Vec3>,
    pub face_indexes: Vec<(usize, usize, usize)>,
}

pub struct Asset {
    pub objects: Vec<Object>,
    pub models: Vec<Model>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Asset {
    pub fn new(file_name: &str) -> Asset {
        let mut asset = Asset {
            objects: vec![],
            models: vec![],
            lights: vec![],
        };

        let (models, materials) = tobj::load_obj(&file_name, true).expect("Failed to load file");

        let scale = 555.0;

        for model in models.iter() {
            let mesh = &model.mesh;
            let mut vertices: Vec<Vec3> = vec![];
            for v in 0..mesh.positions.len() / 3 {
                vertices.push(Vec3::new(
                    mesh.positions[3 * v] as f64,
                    mesh.positions[3 * v + 1] as f64,
                    mesh.positions[3 * v + 2] as f64,
                ));
            }

            let mut triangles: Vec<Triangle> = vec![];
            let mut next_face = 0;

            for f in 0..mesh.num_face_indices.len() {
                let end = next_face + mesh.num_face_indices[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                let triangle = Triangle::new(
                    vertices[*face_indices[0] as usize],
                    vertices[*face_indices[1] as usize],
                    vertices[*face_indices[2] as usize],
                    scale,
                );
                triangles.push(triangle);
                next_face = end;
            }

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
                        Material::Emissive(Emissive::new(m.ambient[0] as f64, diffuse))
                    } else {
                        let ambient_brdf = Lambertian::new(0.5, ambient);
                        let diffuse_brdf = Lambertian::new(1.0, diffuse);
                        Material::Matte(Matte::new(ambient_brdf, diffuse_brdf))
                    };
                    if let Material::Emissive(emissive) = material {
                        let arealight = AreaLight::new(
                            triangles
                                .iter()
                                .map(|t| Box::new(*t) as Box<dyn GeometricObject>)
                                .collect(),
                            emissive,
                        );
                        asset.lights.push(Box::new(arealight));
                    }

                    asset.models.push(Model::new(
                        model.name.clone(),
                        Box::new(material),
                        triangles
                            .iter()
                            .map(|t| Box::new(*t) as Box<dyn GeometricObject>)
                            .collect(),
                    ));
                }
            };
        }
        return asset;
    }
}
