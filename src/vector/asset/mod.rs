/// The [`AssetLoader`](bevy::asset::AssetLoader) for [`SvgVectorAsset`]s.
pub mod loader;

use crate::error::{Result, SvgError};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Transform, Tree},
};

/// An [`Asset`] containing an [`SVG`](https://en.wikipedia.org/wiki/SVG) file
/// losslessly[^1] deserialised as a [`Tree`] container.
///
/// [^1]: Only lossless for the static SVG subset.
#[derive(TypePath, Asset)]
pub struct SvgVectorAsset(pub Tree);

impl SvgVectorAsset {
    /// Renders and rasterises an [`SvgVectorAsset`] containing a [`Tree`] into
    /// a [`Pixmap`] using [`resvg`]'s [`render`](resvg::render) function.
    ///
    /// The rendered [`Pixmap`] is the same size as the SVG file's
    /// [`viewBox`](https://svgwg.org/svg2-draft/coords.html#ViewBoxAttribute).
    /// However, this may change in the future to allow higher-quality
    /// rasterisation of the SVG files.
    ///
    /// ## Errors
    ///
    /// The `viewBox` *must not* be 0 on any axis. If this invariant is broken,
    /// then this function will return an [`SvgError::Empty`].
    pub fn render(&self) -> Result<Pixmap> {
        let (width, height) = self.0.size().to_int_size().dimensions();
        let mut buf = Pixmap::new(width, height).ok_or(SvgError::Empty)?;

        resvg::render(&self.0, Transform::default(), &mut buf.as_mut());
        Ok(buf)
    }

    /// Renders and rasterises an [`SvgVectorAsset`] containing a [`Tree`] into
    /// an [`Image`] using the [`render`](Self::render) method.
    ///
    /// The rendered [`Image`] is the same size as the SVG file's
    /// [`viewBox`](https://svgwg.org/svg2-draft/coords.html#ViewBoxAttribute).
    /// However, this may change in the future to allow higher-quality
    /// rasterisation of the SVG files.
    ///
    /// ## Errors
    ///
    /// The `viewBox` *must not* be 0 on any axis. If this invariant is broken,
    /// then this function will return an [`SvgError::Empty`].
    pub fn render_to_image(&self, asset_usage: RenderAssetUsages) -> Result<Image> {
        let pixmap = self.render()?;
        let (width, height) = self.0.size().to_int_size().dimensions();
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        Ok(Image::new(
            size,
            TextureDimension::D2,
            pixmap.take(),
            TextureFormat::Rgba8Unorm,
            asset_usage,
        ))
    }
}
