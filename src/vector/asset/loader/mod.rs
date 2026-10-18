/// [`AssetLoader`](bevy::asset::AssetLoader) settings for
/// [`SvgFile`](crate::prelude::SvgFile)
pub mod settings;

use crate::{error::SvgError, prelude::SvgAssetLoaderSettings, vector::asset::SvgAsset};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use resvg::usvg::Tree;

/// The [`AssetLoader`] for [`SvgAsset`]s.
///
/// Loads an [`SVG`](https://en.wikipedia.org/wiki/SVG) file into an
/// [`SvgAsset`] containing a [`Tree`].
#[derive(Default, TypePath)]
pub struct SvgAssetLoader;

impl AssetLoader for SvgAssetLoader {
    type Asset = SvgAsset;
    type Settings = SvgAssetLoaderSettings;
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
            let tree = Tree::from_data(&buf, &settings.options.clone().into())?;
            Ok(SvgAsset(tree))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg", "svgz"]
    }
}
