use crate::vector::asset::{SvgVectorAsset, loader::SvgVectorAssetLoader};
use bevy::prelude::*;

pub struct SvgVectorPlugin;

impl Plugin for SvgVectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgVectorAsset>()
            .init_asset_loader::<SvgVectorAssetLoader>();
    }
}
