pub mod loader;

use bevy::prelude::*;
use resvg::usvg::Tree;

#[derive(TypePath, Asset)]
pub struct SvgAsset(pub Tree);
