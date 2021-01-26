extern crate tobj;

use crate::brdf::Lambertian;
use crate::hittable::{Hittable, Triangle};
use crate::material::{Material, Matte};
use crate::model::{Color, Model, Vec3};

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub vertices: Vec<Vec3>,
    pub face_indexes: Vec<(usize, usize, usize)>,
}

pub struct Asset {
    pub objects: Vec<Box<Object>>,
    pub models: Vec<Model>,
}

impl Asset {
    pub fn new(file_name: &str) -> Asset {
        let mut asset = Asset {
            objects: vec![],
            models: vec![],
        };

        let (models, materials) = tobj::load_obj(&file_name, true).expect("Failed to load file");

        for m in models.iter() {
            let mesh = &m.mesh;
            let mut vertices: Vec<Vec3> = vec![];
            for v in 0..mesh.positions.len() / 3 {
                vertices.push(Vec3::new(
                    mesh.positions[3 * v] as f64,
                    mesh.positions[3 * v + 1] as f64,
                    mesh.positions[3 * v + 2] as f64,
                ));
            }

            let mut triangles: Vec<Box<dyn Hittable>> = vec![];
            let mut next_face = 0;

            for f in 0..mesh.num_face_indices.len() {
                let end = next_face + mesh.num_face_indices[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                let triangle = Triangle(
                    vertices[*face_indices[0] as usize],
                    vertices[*face_indices[1] as usize],
                    vertices[*face_indices[2] as usize],
                );
                triangles.push(Box::new(triangle) as Box<dyn Hittable>);
                next_face = end;
            }

            match mesh.material_id {
                None => {}
                Some(material_id) => {
                    let m = &materials[material_id];
                    let ambient_brdf = Lambertian::new(
                        0.5,
                        Color::new(
                            m.ambient[0] as f64,
                            m.ambient[1] as f64,
                            m.ambient[2] as f64,
                        ),
                    );
                    let diffuse_brdf = Lambertian::new(
                        1.0,
                        Color::new(
                            m.diffuse[0] as f64,
                            m.diffuse[1] as f64,
                            m.diffuse[2] as f64,
                        ),
                    );
                    let matte = Matte::new(ambient_brdf, diffuse_brdf);
                    asset.models.push(Model::new(
                        555.0,
                        Box::new(Material::Matte(matte)),
                        triangles,
                    ));
                }
            };
        }
        return asset;
    }
}
