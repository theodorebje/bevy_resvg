use crate::raster::asset::SvgRasterAsset;
use bevy::prelude::*;

/// The [`Component`] that one needs to wrap [`SvgRasterAsset`]s in before
/// spawning them.
#[derive(Component, Default)]
pub struct SvgRasterComponent(pub Handle<SvgRasterAsset>);
