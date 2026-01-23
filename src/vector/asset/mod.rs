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

#[derive(TypePath, Asset)]
pub struct SvgVectorAsset(pub Tree);

impl SvgVectorAsset {
    pub fn render(&self) -> Result<Pixmap> {
        let (width, height) = self.0.size().to_int_size().dimensions();
        let mut buf = Pixmap::new(width, height).ok_or(SvgError::Empty)?;

        resvg::render(&self.0, Transform::default(), &mut buf.as_mut());
        Ok(buf)
    }

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
