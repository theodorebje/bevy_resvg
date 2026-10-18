use super::enum_def;
use resvg::usvg::{ImageRendering, ShapeRendering, Size, TextRendering};
use serde::{Deserialize, Serialize};

enum_def! {
    /// Specifies the default shape rendering method.
    ///
    /// Will be used when an SVG element's `shape-rendering` property is set to `auto`.
    ///
    /// Default: `GeometricPrecision`
    pub enum ShapeRenderingDef -> ShapeRendering {
        OptimizeSpeed,
        CrispEdges,
        GeometricPrecision,
    }

    /// Specifies the default text rendering method.
    ///
    /// Will be used when an SVG element's `text-rendering` property is set to `auto`.
    ///
    /// Default: `OptimizeLegibility`
    pub enum TextRenderingDef -> TextRendering {
        OptimizeSpeed,
        OptimizeLegibility,
        GeometricPrecision,
    }

    /// Specifies the default image rendering method.
    ///
    /// Will be used when an SVG element's `image-rendering` property is set to `auto`.
    ///
    /// Default: `OptimizeQuality`
    pub enum ImageRenderingDef -> ImageRendering {
        OptimizeQuality,
        OptimizeSpeed,
        // The following can only appear as presentation attributes.
        Smooth,
        HighQuality,
        CrispEdges,
        Pixelated,
    }
}

/// Default viewport size to assume if there is no `viewBox` attribute and
/// the `width` or `height` attributes are relative.
///
/// Default: `(100., 100.)`
///
/// ## Guarantees
///
/// - Width and height are positive, non-zero and finite.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(remote = "Size")]
pub struct SizeDef {
    #[serde(getter = "Size::width")]
    width: f32,
    #[serde(getter = "Size::height")]
    height: f32,
}

#[allow(clippy::fallible_impl_from)] // `serde::Deserialize` requires an infallible impl.
impl From<SizeDef> for Size {
    fn from(value: SizeDef) -> Self {
        Self::from_wh(value.width, value.height).unwrap()
    }
}
