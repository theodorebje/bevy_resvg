use crate::raster::asset::SvgRasterAsset;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SvgRasterComponent(pub Handle<SvgRasterAsset>);
