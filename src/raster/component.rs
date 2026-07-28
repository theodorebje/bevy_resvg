use crate::raster::asset::SvgFile;
use bevy::{
    asset::{AsAssetId, AssetId},
    ecs::template::FromTemplate,
    prelude::*,
};

/// The [`Component`] that one needs to wrap [`SvgFile`]s in before
/// spawning them.
#[derive(Component, FromTemplate, Clone, Default)]
pub struct Svg(pub Handle<SvgFile>);

impl From<Handle<SvgFile>> for Svg {
    fn from(handle: Handle<SvgFile>) -> Self {
        Self(handle)
    }
}

impl AsAssetId for Svg {
    type Asset = SvgFile;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}

/// The [`Component`] that one needs to wrap [`SvgFile`]s in before
/// using them in Bevy UIs.
#[derive(Component, FromTemplate, Clone, Default)]
pub struct UiSvg(pub Handle<SvgFile>);

impl From<Handle<SvgFile>> for UiSvg {
    fn from(handle: Handle<SvgFile>) -> Self {
        Self(handle)
    }
}

impl AsAssetId for UiSvg {
    type Asset = SvgFile;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}
