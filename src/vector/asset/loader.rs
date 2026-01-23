use crate::{error::SvgError, vector::asset::SvgVectorAsset};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use resvg::usvg::{Options, Tree};

/// The [`AssetLoader`] for [`SvgVectorAsset`]s.
///
/// Loads an [`SVG`](https://en.wikipedia.org/wiki/SVG) file into an
/// [`SvgVectorAsset`] containing a [`Tree`].
#[derive(Default, TypePath)]
pub struct SvgVectorAssetLoader;

impl AssetLoader for SvgVectorAssetLoader {
    type Asset = SvgVectorAsset;
    type Settings = ();
    type Error = SvgError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf).await?;
            let options = Options::default();
            let tree = Tree::from_data(&buf, &options)?;
            Ok(SvgVectorAsset(tree))
        })
    }
}
