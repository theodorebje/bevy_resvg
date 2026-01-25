use crate::{error::SvgError, raster::asset::SvgFile, vector::asset::loader::SvgVectorAssetLoader};
use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};

/// The [`AssetLoader`] for [`SvgFile`]s.
///
/// Loads an [`SVG`](https://en.wikipedia.org/wiki/SVG) file using an
/// [`SvgVectorAssetLoader`] into an
/// [`SvgVectorAsset`](crate::vector::asset::SvgVectorAsset), and then
/// rasterises it into a [`SvgFile`] containing an [`Image`] using
/// [`resvg`]'s [`render`](resvg::render) function.
#[derive(Default, TypePath)]
pub struct SvgFileLoader;

impl AssetLoader for SvgFileLoader {
    type Asset = SvgFile;
    type Settings = ();
    type Error = SvgError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let vector_asset = SvgVectorAssetLoader
                .load(reader, settings, load_context)
                .await?;

            let image = vector_asset.render_to_image(RenderAssetUsages::default())?;

            Ok(SvgFile(image))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg", "svgz"]
    }
}
