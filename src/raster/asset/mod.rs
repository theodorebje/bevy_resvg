pub mod loader;

use bevy::prelude::*;

#[derive(TypePath, Asset)]
pub struct SvgRasterAsset(pub Image);
