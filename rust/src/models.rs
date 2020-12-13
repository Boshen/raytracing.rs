use nalgebra::{Vector3};

use crate::model::{Model, Material, Triangle, Sphere, Hittable};

pub fn default_metarial() -> Material {
    return Material {
        diffuse_reflection: 1.0,
        diffuse_color: Vector3::new(0.0, 0.0, 0.0),
        reflection: 0.0,
        specular_refection: 0.0,
        shininess: 0.0,
        transparent: false,
    }
}

pub fn get_models() -> Vec<Model> {
    let mut models: Vec<Model> = Vec::new();

    let l = 555.0;
    let z_front = -l;
    let mut a = Vector3::new(l, 0.0, z_front);
    let mut b = Vector3::new(0.0, 0.0, z_front);
    let mut c = Vector3::new(l, 0.0, l);
    let mut d = Vector3::new(0.0, 0.0, l);
    let mut e = Vector3::new(l, l, z_front);
    let mut f = Vector3::new(0.0, l, z_front);
    let mut g = Vector3::new(l, l, l);
    let mut h = Vector3::new(0.0, l, l);

    let material = default_metarial();

    let wall_beige         = Material { diffuse_color: Vector3::new(0.85, 0.85, 0.7), ..material };
    let wall_red           = Material { diffuse_color: Vector3::new(0.75, 0.15, 0.15), ..material };
    let wall_green         = Material { diffuse_color: Vector3::new(0.15, 0.75, 0.15), ..material };
    let light_material     = Material { diffuse_color: Vector3::new(1.0, 1.0, 1.0), diffuse_reflection: 10.0, transparent: true, ..material };
    let light_box_material = Material { diffuse_color: Vector3::new(0.2, 0.2, 0.2), diffuse_reflection: 5.0, transparent: true, ..material };
    let block_blue         = Material { diffuse_color: Vector3::new(0.05, 0.6, 1.0), ..material };
    let block_orange       = Material { diffuse_color: Vector3::new(0.8, 0.7, 0.05), ..material };
    let sphere_material    = Material { diffuse_reflection: 0.0, reflection: 1.0, specular_refection: 1.0, shininess: 5.0, ..material };

    // floor
    models.push(Model::new(wall_beige, vec![t(c, b, a), t(c, d, b)]));
    // left
    models.push(Model::new(wall_red, vec![t(a, e, c), t(c, e, g)]));
    // right
    models.push(Model::new(wall_green, vec![t(f, b, d), t(h, f, d)]));
    // front wall
    models.push(Model::new(wall_beige, vec![t(g, d, c), t(g, h, d)]));

    let hole_radius = 75.0;
    let i = Vector3::new(l / 2.0 + hole_radius, l, l / 2.0 - hole_radius);
    let j = Vector3::new(l / 2.0 - hole_radius, l, l / 2.0 - hole_radius);
    let k = Vector3::new(l / 2.0 + hole_radius, l, l / 2.0 + hole_radius);
    let l2= Vector3::new(l / 2.0 - hole_radius, l, l / 2.0 + hole_radius);
    let mut m = Vector3::new(l / 2.0 + hole_radius, l, z_front);
    let mut n = Vector3::new(l / 2.0 - hole_radius, l, z_front);
    let mut o = Vector3::new(l / 2.0 + hole_radius, l, l + 5.0);
    let mut p = Vector3::new(l / 2.0 - hole_radius, l, l + 5.0);
    e = Vector3::new(l + 5.0, l, z_front);
    f = Vector3::new(-5.0, l, z_front);
    g = Vector3::new(l + 5.0, l, l + 5.0);
    h = Vector3::new(-5.0, l, l + 5.0);

    // ceiling
    models.push(Model::new(wall_beige, vec![
            t(e, m, g),
            t(m, o, g),
            t(m, n, i),
            t(n, j, i),
            t(n, f, p),
            t(f, h, p),
            t(k, l2, o),
            t(l2, p, o),
    ]));

    // light hole
    models.push(Model::new(light_material, vec![t(l2, k, i), t(l2, i, j)]));

    // frame around light
    let light_box_height = 5.0;
    m = Vector3::new(l / 2.0 + hole_radius, l - light_box_height, l / 2.0 - hole_radius);
    n = Vector3::new(l / 2.0 - hole_radius, l - light_box_height, l / 2.0 - hole_radius);
    o = Vector3::new(l / 2.0 + hole_radius, l - light_box_height, l / 2.0 + hole_radius);
    p = Vector3::new(l / 2.0 - hole_radius, l - light_box_height, l / 2.0 + hole_radius);
    models.push(Model::new(light_box_material, vec![
            t(i, j, m),
            t(j, n, m),
            t(j, l2, n),
            t(l2, p, n),
            t(l2, k, o),
            t(l2, o, p),
            t(i, m, o),
            t(k, i, o),
    ]));

    // short block
    a = Vector3::new(290.0, 0.0, 114.0);
    b = Vector3::new(130.0, 0.0, 65.0);
    c = Vector3::new(240.0, 0.0, 272.0);
    d = Vector3::new(82.0, 0.0, 225.0);
    e = Vector3::new(290.0, 165.0, 114.0);
    f = Vector3::new(130.0, 165.0, 65.0);
    g = Vector3::new(240.0, 165.0, 272.0);
    h = Vector3::new(82.0, 165.0, 225.0);

    models.push(Model::new(block_blue, vec![
            t(e, b, a),
            t(e, f, b),
            t(f, d, b),
            t(f, h, d),
            t(h, c, d),
            t(h, g, c),
            t(g, e, c),
            t(e, a, c),
            t(g, f, e),
            t(g, h, f),
    ]));

    // tall block
    a = Vector3::new(423.0, 0.0, 247.0);
    b = Vector3::new(265.0, 0.0, 296.0);
    c = Vector3::new(472.0, 0.0, 406.0);
    d = Vector3::new(314.0, 0.0, 456.0);
    e = Vector3::new(423.0, 330.0, 247.0);
    f = Vector3::new(265.0, 330.0, 296.0);
    g = Vector3::new(472.0, 330.0, 406.0);
    h = Vector3::new(314.0, 330.0, 456.0);

    models.push(Model::new(block_orange, vec![
            t(e, b, a),
            t(e, f, b),
            t(f, d, b),
            t(f, h, d),
            t(h, c, d),
            t(h, g, c),
            t(g, e, c),
            t(e, a, c),
            t(g, f, e),
            t(g, h, f),
    ]));

    // sphere
    models.push(Model::new(sphere_material, vec![
            Box::new(Sphere::new(40.0, Vector3::new(200.0, 165.0 + 40.0, 120.0)))
    ]));

    for m in &mut models {
        m.scale(l)
    }
    models
}

fn t(a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>) -> Box<dyn Hittable> {
    Box::new(Triangle(a, b, c))
}

