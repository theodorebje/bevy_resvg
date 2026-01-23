/// The [`AssetLoader`](bevy::asset::AssetLoader) for [`SvgRasterAsset`]s.
pub mod loader;

use bevy::prelude::*;

/// An [`Asset`] containing an [`SVG`](https://en.wikipedia.org/wiki/SVG) file
/// rendered into an [`Image`].
#[derive(TypePath, Asset)]
pub struct SvgRasterAsset(pub Image);
