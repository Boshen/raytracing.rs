use enum_dispatch::enum_dispatch;

pub mod simple;
pub mod thin_lens;

use crate::world::World;
pub use simple::*;
pub use thin_lens::*;

#[enum_dispatch]
pub enum CameraEnum {
    SimpleCamera,
    ThinLensCamera,
}

#[enum_dispatch(CameraEnum)]
pub trait Camera {
    fn render_scene(&self, world: &World) -> Vec<u8>;
}
