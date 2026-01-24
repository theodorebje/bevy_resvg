/// The [`AssetLoader`](bevy::asset::AssetLoader) for [`SvgFile`]s.
pub(crate) mod loader;

use bevy::prelude::*;

/// An [`Asset`] containing an [`SVG`](https://en.wikipedia.org/wiki/SVG) file
/// rendered into an [`Image`].
#[derive(TypePath, Asset)]
pub struct SvgFile(pub Image);
