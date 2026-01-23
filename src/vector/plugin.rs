use crate::vector::asset::{SvgVectorAsset, loader::SvgVectorAssetLoader};
use bevy::prelude::*;

/// The [`Plugin`] for initialising the
/// [Vector](https://en.wikipedia.org/wiki/Vector_graphics)
/// [`Asset`] and [`AssetLoader`](bevy::asset::AssetLoader).
pub struct SvgVectorPlugin;

impl Plugin for SvgVectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgVectorAsset>()
            .init_asset_loader::<SvgVectorAssetLoader>();
    }
}
