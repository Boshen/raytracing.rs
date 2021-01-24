use crate::brdf::{GlossySpecular, Lambertian, PerfectSpecular};
use crate::hittable::{Hittable, Sphere, Triangle};
use crate::material::{Material, Matte, Phong, Reflective};
use crate::model::{Color, Model, Vec3};

fn matte(cd: Color) -> Box<Material> {
    let diffuse_brdf = Lambertian::new(1.0, cd);
    let ambient_brdf = Lambertian::new(1.0, Color::new(1.0, 1.0, 1.0));
    let matte = Matte::new(Box::new(ambient_brdf), Box::new(diffuse_brdf));
    return Box::new(Material::Matte(matte));
}

pub fn get_models() -> Vec<Model> {
    let mut models: Vec<Model> = Vec::new();

    let l = 555.0;
    let z_front = -l;
    let mut a = Vec3::new(l, 0.0, z_front);
    let mut b = Vec3::new(0.0, 0.0, z_front);
    let mut c = Vec3::new(l, 0.0, l);
    let mut d = Vec3::new(0.0, 0.0, l);
    let mut e = Vec3::new(l, l, z_front);
    let mut f = Vec3::new(0.0, l, z_front);
    let mut g = Vec3::new(l, l, l);
    let mut h = Vec3::new(0.0, l, l);

    let wall_beige0 = matte(Color::new(0.85, 0.85, 0.7));
    let wall_beige1 = matte(Color::new(0.85, 0.85, 0.7));
    let wall_beige2 = matte(Color::new(0.85, 0.85, 0.7));
    let wall_beige3 = matte(Color::new(0.85, 0.85, 0.7));
    let wall_red = matte(Color::new(0.75, 0.15, 0.15));
    let wall_green = matte(Color::new(0.15, 0.75, 0.15));
    // let light_material = matte(Color::new(1.0, 1.0, 1.0));
    // let light_box_material = matte(Color::new(0.2, 0.2, 0.2));
    let block_blue = matte(Color::new(0.05, 0.6, 1.0));
    let block_orange = matte(Color::new(0.8, 0.7, 0.05));
    let sphere1_material = Box::new(Material::Phong(Phong::new(
        Box::new(Lambertian::new(1.0, Color::new(1.0, 1.0, 1.0))),
        Box::new(Lambertian::new(1.0, Color::new(0.8, 0.8, 0.8))),
        Box::new(GlossySpecular::new(0.15, 1.0)),
    )));
    let sphere2_material = Box::new(Material::Reflective(Reflective::new(
        Box::new(Lambertian::new(1.0, Color::new(1.0, 1.0, 1.0))),
        Box::new(Lambertian::new(1.0, Color::new(0.8, 0.8, 0.8))),
        Box::new(GlossySpecular::new(0.15, 1.0)),
        Box::new(PerfectSpecular::new(0.75, Color::new(1.0, 1.0, 1.0))),
    )));

    // floor
    models.push(Model::new(l, wall_beige0, vec![t(c, b, a), t(c, d, b)]));
    // left
    models.push(Model::new(l, wall_red, vec![t(a, e, c), t(c, e, g)]));
    // right
    models.push(Model::new(l, wall_green, vec![t(f, b, d), t(h, f, d)]));
    // front wall
    models.push(Model::new(l, wall_beige1, vec![t(g, d, c), t(g, h, d)]));
    // wall behind camera
    models.push(Model::new(l, wall_beige2, vec![t(f, e, a), t(f, a, b)]));

    let hole_radius = 75.0;
    let i = Vec3::new(l / 2.0 + hole_radius, l, l / 2.0 - hole_radius);
    let j = Vec3::new(l / 2.0 - hole_radius, l, l / 2.0 - hole_radius);
    let k = Vec3::new(l / 2.0 + hole_radius, l, l / 2.0 + hole_radius);
    let l2 = Vec3::new(l / 2.0 - hole_radius, l, l / 2.0 + hole_radius);
    let m = Vec3::new(l / 2.0 + hole_radius, l, z_front);
    let n = Vec3::new(l / 2.0 - hole_radius, l, z_front);
    let o = Vec3::new(l / 2.0 + hole_radius, l, l + 5.0);
    let p = Vec3::new(l / 2.0 - hole_radius, l, l + 5.0);
    e = Vec3::new(l + 5.0, l, z_front);
    f = Vec3::new(-5.0, l, z_front);
    g = Vec3::new(l + 5.0, l, l + 5.0);
    h = Vec3::new(-5.0, l, l + 5.0);

    // ceiling
    models.push(Model::new(
        l,
        wall_beige3,
        vec![
            t(e, m, g),
            t(m, o, g),
            t(m, n, i),
            t(n, j, i),
            t(n, f, p),
            t(f, h, p),
            t(k, l2, o),
            t(l2, p, o),
        ],
    ));

    // light hole
    // models.push(Model::new(
    // l,
    // light_material,
    // vec![t(l2, k, i), t(l2, i, j)],
    // ));

    // frame around light
    // let light_box_height = 5.0;
    // m = Vec3::new(
    // l / 2.0 + hole_radius,
    // l - light_box_height,
    // l / 2.0 - hole_radius,
    // );
    // n = Vec3::new(
    // l / 2.0 - hole_radius,
    // l - light_box_height,
    // l / 2.0 - hole_radius,
    // );
    // o = Vec3::new(
    // l / 2.0 + hole_radius,
    // l - light_box_height,
    // l / 2.0 + hole_radius,
    // );
    // p = Vec3::new(
    // l / 2.0 - hole_radius,
    // l - light_box_height,
    // l / 2.0 + hole_radius,
    // );
    // models.push(Model::new(
    // l,
    // light_box_material,
    // vec![
    // t(i, j, m),
    // t(j, n, m),
    // t(j, l2, n),
    // t(l2, p, n),
    // t(l2, k, o),
    // t(l2, o, p),
    // t(i, m, o),
    // t(k, i, o),
    // ],
    // ));

    // short block
    a = Vec3::new(290.0, 0.0, 114.0);
    b = Vec3::new(130.0, 0.0, 65.0);
    c = Vec3::new(240.0, 0.0, 272.0);
    d = Vec3::new(82.0, 0.0, 225.0);
    e = Vec3::new(290.0, 165.0, 114.0);
    f = Vec3::new(130.0, 165.0, 65.0);
    g = Vec3::new(240.0, 165.0, 272.0);
    h = Vec3::new(82.0, 165.0, 225.0);

    models.push(Model::new(
        l,
        block_blue,
        vec![
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
        ],
    ));

    // tall block
    a = Vec3::new(423.0, 0.0, 247.0);
    b = Vec3::new(265.0, 0.0, 296.0);
    c = Vec3::new(472.0, 0.0, 406.0);
    d = Vec3::new(314.0, 0.0, 456.0);
    e = Vec3::new(423.0, 330.0, 247.0);
    f = Vec3::new(265.0, 330.0, 296.0);
    g = Vec3::new(472.0, 330.0, 406.0);
    h = Vec3::new(314.0, 330.0, 456.0);

    models.push(Model::new(
        l,
        block_orange,
        vec![
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
        ],
    ));

    // sphere
    models.push(Model::new(
        l,
        sphere1_material,
        vec![Box::new(Sphere::new(
            40.0,
            Vec3::new(200.0, 165.0 + 40.0, 120.0),
        ))],
    ));

    models.push(Model::new(
        l,
        sphere2_material,
        vec![Box::new(Sphere::new(60.0, Vec3::new(400.0, 60.0, 100.0)))],
    ));

    models
}

fn t(a: Vec3, b: Vec3, c: Vec3) -> Box<dyn Hittable> {
    Box::new(Triangle(a, b, c))
}
