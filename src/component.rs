use crate::asset::SvgAsset;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SvgComponent(pub Handle<SvgAsset>);
