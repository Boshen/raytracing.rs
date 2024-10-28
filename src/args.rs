use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// Show a quick preview.
    pub preview: bool,

    #[bpaf(fallback(500))]
    pub width: u32,

    #[bpaf(fallback(500))]
    pub height: u32,

    #[bpaf(external(arg_camera), fallback(ArgCamera::ThinLens))]
    pub camera: ArgCamera,

    #[bpaf(fallback(16))]
    pub samples: u8,
}

#[derive(Debug, Clone, Bpaf)]
pub enum ArgCamera {
    Simple,
    ThinLens,
}
