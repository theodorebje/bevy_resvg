use crate::vector::asset::SvgAsset;
use bevy::{
    asset::{AsAssetId, AssetId},
    ecs::template::FromTemplate,
    prelude::*,
};

/// The [`Component`] that one needs to wrap [`SVG`](https://en.wikipedia.org/wiki/SVG) files in before
/// spawning them.
#[derive(Component, FromTemplate, Clone, Default)]
pub struct Svg(pub Handle<SvgAsset>);

impl From<Handle<SvgAsset>> for Svg {
    fn from(handle: Handle<SvgAsset>) -> Self {
        Self(handle)
    }
}

impl AsAssetId for Svg {
    type Asset = SvgAsset;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}

/// The [`Component`] that one needs to wrap [`SvgAsset`]s in before
/// using them in Bevy UIs.
#[derive(Component, FromTemplate, Clone, Default)]
pub struct UiSvg(pub Handle<SvgAsset>);

impl From<Handle<SvgAsset>> for UiSvg {
    fn from(handle: Handle<SvgAsset>) -> Self {
        Self(handle)
    }
}

impl AsAssetId for UiSvg {
    type Asset = SvgAsset;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}
