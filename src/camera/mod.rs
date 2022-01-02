mod setting;
mod simple;
mod thin_lens;

pub use setting::*;
pub use simple::*;
pub use thin_lens::*;

use crate::color::Color;
use crate::world::World;

pub trait Camera {
    fn render_scene(&self, world: &World) -> Vec<Color>;
}
