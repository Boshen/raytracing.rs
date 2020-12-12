use image::{RgbImage, Rgb};
use nalgebra::{Vector3, Dot, Cross};
use std::ops::{Sub, Mul};

struct Ray {
    start: Vector3<f64>,
    direction: Vector3<f64>,
}

struct Triangle(Vector3<f64>, Vector3<f64>, Vector3<f64>);

impl Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let epsilon = 0.000001;
        let e1 = self.1.sub(self.0);
        let e2 = self.2.sub(self.0);

        let h = ray.direction.cross(&e2);
        let a = e1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None
        }

        let f = 1.0 / a;
        let s = ray.start.sub(self.0);
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None
        }

        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = f * e2.dot(&q);
        if t <= epsilon {
            return None
        }

        return Some(t)
    }

    fn scale(&mut self, l: f64) {
        self.0 = self.0.mul(2.0 / l);
        self.1 = self.1.mul(2.0 / l);
        self.2 = self.2.mul(2.0 / l);

        self.0 = self.0.sub(Vector3::new(1.0, 1.0, 1.0));
        self.1 = self.1.sub(Vector3::new(1.0, 1.0, 1.0));
        self.2 = self.2.sub(Vector3::new(1.0, 1.0, 1.0));

        self.0.x = -self.0.x;
        self.1.x = -self.1.x;
        self.2.x = -self.2.x;

        self.0.y = -self.0.y;
        self.1.y = -self.1.y;
        self.2.y = -self.2.y;
    }
}

fn get_models() -> Vec<Triangle> {
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

fn trace(models: &Vec<Triangle>, ray: &Ray) -> Vector3<f64> {

    let mut min_distance = f64::INFINITY;
    let mut hit_model: Option<&Triangle> = None;
    for m in models {
        match m.intersects(ray) {
            None => (),
            Some(t) => {
                if t < min_distance {
                    min_distance = t;
                    hit_model = Some(&m)
                }
            }
        }
    }

    match hit_model {
        None => Vector3::new(0.0, 0.0, 0.0),
        Some(_m) => Vector3::new(1.0, 1.0, 1.0),
    }
}

fn main() {
    let width = 500;
    let height = 500;
    let focal_length = width;
    let mut image = RgbImage::new(width, height);

    let camera = Vector3::new(0.0, 0.0, -3.0);

    let models = get_models();

    for i in 0..width {
        for j in 0..height {
            let x = (i as f64)  - (width as f64) / 2.0;
            let y = (j as f64) - (height as f64) / 2.0;
            let d = Vector3::new(x, y, focal_length as f64);
            let ray = Ray {start: camera, direction: d};
            let color = trace(&models, &ray);
            let rgb = Rgb([
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8
            ]);
            image.put_pixel(i, j, rgb);
        }
    }

    image.save("output.png").unwrap();
}
