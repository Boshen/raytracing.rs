pub mod aabb;
pub mod accelerator;
pub mod args;
pub mod asset;
pub mod brdf;
pub mod camera;
pub mod color;
pub mod geometric_object;
pub mod light;
pub mod material;
pub mod model;
pub mod ray;
pub mod renderer;
pub mod sampler;
pub mod scene;

#[cfg(test)]
mod tests {
    use crate::{
        args::{ArgCamera, Args},
        renderer::Renderer,
        scene::CornellBox,
    };

    #[test]
    fn render() {
        let args =
            Args { width: 10, height: 10, preview: false, camera: ArgCamera::ThinLens, samples: 4 };
        let scene = CornellBox::new(args.width, args.height, &args);
        let renderer = Renderer::new(scene, &args);
        let pixels = renderer.render();
        assert_eq!(pixels.len(), 100);
    }
}
