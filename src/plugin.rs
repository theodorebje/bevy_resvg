use crate::{raster::plugin::SvgRasterPlugin, vector::plugin::SvgVectorPlugin};
use bevy::prelude::*;

/// A combination of the [`SvgVectorPlugin`] and the [`SvgRasterPlugin`] for
/// easier use of this crate.
#[derive(Debug, Clone, Copy)]
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((SvgRasterPlugin, SvgVectorPlugin));
    }
}
