/// The [`AssetLoader`](bevy::asset::AssetLoader) for [`SvgAsset`]s.
pub mod loader;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use resvg::{
    tiny_skia::{IntSize, Pixmap},
    usvg::{Transform, Tree},
};

/// An [`Asset`] containing an [`SVG`](https://en.wikipedia.org/wiki/SVG) file
/// deserialised visually losslessly[^1] as a [`Tree`] container.
///
/// [^1]: Only visually lossless for the static SVG subset.
#[derive(TypePath, Asset)]
pub struct SvgAsset(pub Tree);

impl SvgAsset {
    /// Renders and rasterises an [`SvgAsset`] containing a [`Tree`] into an [`Image`] using
    /// [`resvg`]'s [`render`](resvg::render) function.
    pub(crate) fn render_to_image(&self, size: IntSize) -> Image {
        let (width, height) = size.dimensions();
        let mut pixmap = Pixmap::new(width, height).unwrap_or_else(|| unreachable!());

        let original_size = self.0.size();
        #[expect(clippy::cast_precision_loss, reason = "we don't care")]
        let (sx, sy) = (
            width as f32 / original_size.width(),
            height as f32 / original_size.height(),
        );

        let transform = Transform::from_scale(sx, sy);
        resvg::render(&self.0, transform, &mut pixmap.as_mut());

        let size = Extent3d {
            width: pixmap.width(),
            height: pixmap.height(),
            // No clue what this does
            depth_or_array_layers: 1,
        };
        Image::new(
            size,
            TextureDimension::D2,
            pixmap.take(),
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD,
        )
    }
}
