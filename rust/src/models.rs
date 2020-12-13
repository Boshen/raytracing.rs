use nalgebra::{Vector3};

use crate::model::{Triangle};

pub fn get_models() -> Vec<Triangle> {
    let mut models: Vec<Triangle> = Vec::new();
    let l = 555.0;
    let z_front = -l;
    let a = Vector3::new(l, 0.0, z_front);
    let b = Vector3::new(0.0, 0.0, z_front);
    let c = Vector3::new(l, 0.0, l);
    let d = Vector3::new(0.0, 0.0, l);
    let e = Vector3::new(l, l, z_front);
    let f = Vector3::new(0.0, l, z_front);
    let g = Vector3::new(l, l, l);
    let h = Vector3::new(0.0, l, l);

    models.push(Triangle(c, b, a));
    models.push(Triangle(c, d, b));
    models.push(Triangle(a, e, c));
    models.push(Triangle(c, e, g));
    models.push(Triangle(f, b, d));
    models.push(Triangle(h, f, d));
    models.push(Triangle(g, d, c));
    models.push(Triangle(g, h, d));
    for m in &mut models {
        m.scale(l)
    }
    models
}

