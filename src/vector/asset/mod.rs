/// The [`AssetLoader`](bevy::asset::AssetLoader) for [`SvgAsset`]s.
pub mod loader;

use crate::{
    error::{Result, SvgError},
    vector::asset::loader::settings::TargetRenderSize,
};
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
pub struct SvgAsset(pub Tree);

impl From<&SvgAsset> for TargetRenderSize {
    fn from(value: &SvgAsset) -> Self {
        let (width, height) = value.0.size().to_int_size().dimensions();
        Self { width, height }
    }
}

impl From<TargetRenderSize> for Extent3d {
    fn from(value: TargetRenderSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
            depth_or_array_layers: 1,
        }
    }
}

impl SvgAsset {
    /// Renders and rasterises an [`SvgAsset`] containing a [`Tree`] into
    /// a [`Pixmap`] using [`resvg`]'s [`render`](resvg::render) function.
    ///
    /// ## Precision loss
    ///
    /// If the [`TargetRenderSize`] has a value higher than 2^24 on any axis, it
    /// will be rounded to the nearest power of 2. This is due to [`f32`] having
    /// a 23-bit mantissa, which cannot fit the 32-bit [`u32`]'s in them.
    ///
    /// ## Errors
    ///
    /// The `TargetRenderSize` *must not* be 0 on any axis. If this invariant is
    /// broken, then this function will return an [`SvgError::Empty`].
    fn render(&self, size: TargetRenderSize) -> Result<Pixmap> {
        let TargetRenderSize { width, height } = size;
        let mut buf = Pixmap::new(width, height).ok_or(SvgError::Empty)?;

        let original_size = self.0.size();
        let (original_width, original_height) = (original_size.width(), original_size.height());

        #[allow(clippy::cast_precision_loss)] // See ## Precision loss
        let (scale_x, scale_y) = (
            width as f32 / original_width,
            height as f32 / original_height,
        );

        let transform = Transform::from_scale(scale_x, scale_y);

        resvg::render(&self.0, transform, &mut buf.as_mut());
        Ok(buf)
    }

    /// Renders and rasterises an [`SvgAsset`] containing a [`Tree`] into
    /// an [`Image`] using the [`render`](Self::render) method.
    ///
    /// If `size` is set to `None`, then the `Size` will be set to the SVG
    /// file's
    /// [`viewBox`](https://svgwg.org/svg2-draft/coords.html#ViewBoxAttribute).
    ///
    /// ## Errors
    ///
    /// The `TargetRenderSize` (or `viewBox`, if `None`) *must not* be 0 on any
    /// axis. If this invariant is broken, then this function will return an
    /// [`SvgError::Empty`].
    pub fn render_to_image(
        &self,
        size: Option<TargetRenderSize>,
        asset_usage: RenderAssetUsages,
    ) -> Result<Image> {
        let size = size.unwrap_or_else(|| TargetRenderSize::from(self));
        let pixmap = self.render(size)?;
        Ok(Image::new(
            size.into(),
            TextureDimension::D2,
            pixmap.take(),
            TextureFormat::Rgba8Unorm,
            asset_usage,
        ))
    }
}
