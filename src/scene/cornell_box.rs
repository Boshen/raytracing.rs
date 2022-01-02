use std::sync::Arc;

use crate::args::{ArgCamera, Args};
use crate::asset::Asset;
use crate::camera::{Camera, Setting, Simple, ThinLens};
use crate::geometric_object::{BvhNode, Geometry};
use crate::light::Light;
use crate::light::{Ambient, AmbientOcculuder};
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
        let asset = Asset::new("./assets/cornell_box.obj");
        let ambient_light = Arc::new(Ambient {
            ls: 0.1,
            cl: Vec3::repeat(1.0),
        });

        let mut lights = asset.lights;

        let ambient_occuluder = Arc::new(AmbientOcculuder::new(1.0, Vec3::repeat(1.0)));
        lights.push(ambient_occuluder);

        for light in &mut lights {
            if let Some(l) = Arc::get_mut(light) {
                l.set_sample_points_sqrt(if args.preview { 1 } else { 8 });
            }
        }

        let mut camera_setting =
            Setting::new(Pot3::new(0.0, 0.0, -3.0), Pot3::new(0.0, 0.0, 0.0), 500.0);
        camera_setting.set_view((view_width, view_height));
        camera_setting.set_sample_points_sqrt(if args.preview { 1 } else { 8 });

        let camera: Box<dyn Camera> = match args.camera {
            ArgCamera::Simple => Box::new(Simple::new(camera_setting)),
            ArgCamera::ThinLens => Box::new(ThinLens::new(camera_setting, 0.001, 500.0)),
        };

        let root = BvhNode::construct(asset.geometries);

        Self {
            view_width,
            view_height,
            camera,
            ambient_light,
            lights,
            root,
        }
    }
}
