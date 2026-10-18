use crate::{
    error::SvgError, prelude::SvgAssetLoaderSettings, raster::asset::SvgFile,
    vector::asset::loader::SvgAssetLoader,
};
use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};

/// The [`AssetLoader`] for [`SvgFile`]s.
///
/// Loads an [`SVG`](https://en.wikipedia.org/wiki/SVG) file using an
/// [`SvgAssetLoader`] into an
/// [`SvgAsset`](crate::vector::asset::SvgAsset), and then
/// rasterises it into a [`SvgFile`] containing an [`Image`] using
/// [`resvg`]'s [`render`](resvg::render) function.
#[derive(Default, TypePath)]
pub struct SvgFileLoader;

impl AssetLoader for SvgFileLoader {
    type Asset = SvgFile;
    type Settings = SvgAssetLoaderSettings;
    type Error = SvgError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let vector_asset = SvgAssetLoader.load(reader, settings, load_context).await?;

            let image = vector_asset
                .render_to_image(settings.target_render_size, RenderAssetUsages::default())?;

            Ok(SvgFile(image))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg", "svgz"]
    }
}
