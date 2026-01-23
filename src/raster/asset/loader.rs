use crate::{
    error::SvgError, raster::asset::SvgRasterAsset, vector::asset::loader::SvgVectorAssetLoader,
};
use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};

#[derive(Default, TypePath)]
pub struct SvgRasterAssetLoader;

impl AssetLoader for SvgRasterAssetLoader {
    type Asset = SvgRasterAsset;
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

            Ok(SvgRasterAsset(image))
        })
    }
}
