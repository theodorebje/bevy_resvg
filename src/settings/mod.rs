use resvg::usvg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SvgFileLoaderSettings {
    pub target_render_size: Option<TargetRenderSize>,
    /// Load-time options for [`SvgFile`](crate::prelude::SvgFile).
    pub options: usvg::Options<'static>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TargetRenderSize {
    pub width: u32,
    pub height: u32,
}
