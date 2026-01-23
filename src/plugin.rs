use crate::asset::{SvgAsset, loader::SvgAssetLoader};
use bevy::{app::Plugin, asset::AssetApp};

#[derive(Debug, Clone, Copy)]
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_asset::<SvgAsset>()
            .init_asset_loader::<SvgAssetLoader>();
    }
}
