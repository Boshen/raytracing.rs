use std::sync::Arc;

use crate::args::{ArgCamera, Args};
use crate::asset::Asset;
use crate::brdf::{GlossySpecular, Lambertian, PerfectSpecular};
use crate::camera::{Camera, Pinhole, Setting, ThinLens};
use crate::color::Color;
use crate::geometric_object::{BvhNode, Geometry, Sphere};
use crate::light::{Ambient, AmbientOcculuder, Light};
use crate::material::Reflective;
use crate::model::{Pot3, Vec3};

pub struct CornellBox {
    pub view_width: u32,
    pub view_height: u32,
    pub camera: Box<dyn Camera>,
    pub ambient_light: Arc<Ambient>,
    pub lights: Vec<Arc<dyn Light>>,
    pub root: Arc<dyn Geometry>,
}

impl CornellBox {
    #[must_use]
    pub fn new(view_width: u32, view_height: u32, args: &Args) -> Self {
        let scale = 555.0;
        let mut asset = Asset::new("./assets/cornell_box.obj", scale);

        let ambient_light = Arc::new(Ambient {
            ls: 0.1,
            cl: Vec3::repeat(1.0),
        });

        let ambient_occuluder = Arc::new(AmbientOcculuder::new(1.0, Vec3::repeat(1.0)));
        asset.lights.push(ambient_occuluder);

        let mut camera_setting =
            Setting::new(Pot3::new(0.0, 0.0, -3.0), Pot3::new(0.0, 0.0, 0.0), 500.0);
        camera_setting.set_view((view_width, view_height));

        let camera: Box<dyn Camera> = match args.camera {
            ArgCamera::Simple => Box::new(Pinhole::new(camera_setting)),
            ArgCamera::ThinLens => Box::new(ThinLens::new(camera_setting, 0.001, 500.0)),
        };

        let ball_material = Reflective::new(
            Lambertian::new(0.1, Color::new(1.0, 1.0, 1.0)),
            Lambertian::new(0.7, Color::new(1.0, 1.0, 1.0)),
            GlossySpecular::new(0.2, 3.0),
            // FIXME fix broken reflection on sphere
            PerfectSpecular::new(0.0, Color::new(1.0, 1.0, 1.0)),
        );

        let ball = Arc::new(Sphere::new(
            ball_material,
            40.0,
            Pot3::new(400.0, 40.0, 500.0),
            scale,
        ));

        asset.geometries.push(ball);
        let root = BvhNode::construct(asset.geometries);

        Self {
            view_width,
            view_height,
            camera,
            ambient_light,
            lights: asset.lights,
            root,
        }
    }
}
