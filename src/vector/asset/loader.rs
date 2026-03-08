use crate::{error::SvgError, prelude::SvgFileLoaderSettings, vector::asset::SvgVectorAsset};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use resvg::usvg::Tree;

/// The [`AssetLoader`] for [`SvgVectorAsset`]s.
///
/// Loads an [`SVG`](https://en.wikipedia.org/wiki/SVG) file into an
/// [`SvgVectorAsset`] containing a [`Tree`].
#[derive(Default, TypePath)]
pub struct SvgVectorAssetLoader;

impl AssetLoader for SvgVectorAssetLoader {
    type Asset = SvgVectorAsset;
    type Settings = SvgFileLoaderSettings;
    type Error = SvgError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf).await?;
            let tree = Tree::from_data(&buf, &settings.options)?;
            Ok(SvgVectorAsset(tree))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg", "svgz"]
    }
}
