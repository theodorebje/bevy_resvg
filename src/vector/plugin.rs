use crate::vector::asset::{SvgAsset, loader::SvgAssetLoader};
use bevy::prelude::*;

/// The [`Plugin`] for initialising the
/// [Vector](https://en.wikipedia.org/wiki/Vector_graphics)
/// [`Asset`] and [`AssetLoader`](bevy::asset::AssetLoader).
pub struct SvgVectorPlugin;

impl Plugin for SvgVectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgAsset>()
            .init_asset_loader::<SvgAssetLoader>();
    }
}
