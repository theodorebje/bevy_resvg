use crate::raster::{
    asset::{SvgFile, loader::SvgFileLoader},
    component::Svg,
};
use bevy::prelude::*;

/// The [`Plugin`] for initialising the
/// [Rasterised](https://en.wikipedia.org/wiki/Raster_graphics)
/// [`Asset`] and [`AssetLoader`](bevy::asset::AssetLoader).
pub struct SvgRasterPlugin;

impl Plugin for SvgRasterPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgFile>()
            .init_asset_loader::<SvgFileLoader>()
            .add_systems(Update, spawn_svg_sprites);
    }
}

/// Adds a [`Sprite`] to all [`Entities`](Entity) with an [`Svg`]
/// that doesn't already have an associated [`Sprite`].
fn spawn_svg_sprites(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &Svg), Without<Sprite>>,
) {
    for event in svg_events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } => {
                // Find all entities waiting for this SVG
                for (entity, raster_component) in &query {
                    if raster_component.0.id() == *id
                        && let Some(svg_asset) = svg_assets.get(*id)
                    {
                        let handle = images.add(svg_asset.0.clone());

                        commands.entity(entity).insert(Sprite::from_image(handle));
                    }
                }
            }
            AssetEvent::Added { id } => {
                debug!("added SVG asset '{id}'");
            }
            other => {
                // TODO: Handle more events
                warn!("Unimplemented Event for SVG raster assets: '{other:#?}'");
            }
        }
    }
}
