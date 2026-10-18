/// Types used in [`OptionsDef`]. These are a [Serializable](Serialize) subset
/// of the types used in [`Options`](resvg::usvg::Options).
pub mod options;

use self::options::OptionsDef;
use serde::{Deserialize, Serialize};

/// Settings for the parsing and rendering of an SVG file.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct SvgAssetLoaderSettings {
    /// Optional fixed target size for rendering.
    ///
    /// If `None`, the SVG's
    /// [`viewBox`](https://svgwg.org/svg2-draft/coords.html#ViewBoxAttribute)
    /// determines the size.
    pub target_render_size: Option<TargetRenderSize>,
    /// A [Serializable](Serialize) subset of [`Options`](resvg::usvg::Options).
    pub options: OptionsDef,
}

/// Explicit dimensions of the output image, in pixels. The aspect ratio does
/// not have to be the same as the SVG's
/// [`viewBox`](https://svgwg.org/svg2-draft/coords.html#ViewBoxAttribute).
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TargetRenderSize {
    /// Target width in pixels.
    pub width: u32,
    /// Target height in pixels.
    pub height: u32,
}
