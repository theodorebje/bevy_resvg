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
            .add_systems(
                Update,
                (handle_svg_loaded, handle_svg_modified, handle_svg_removed),
            );
    }
}

macro_rules! read_events {
    ($svg_events:expr, $($asset_event:path) | +) => {
        $svg_events.read().filter_map(|event| match event {
            $($asset_event { id } => Some(*id)),+,
            _ => None,
        }).collect::<Vec<_>>()
    };
}

/// Handles newly loaded [`SvgFile`]s by adding [`Sprite`] components to waiting
/// entities. This responds to [`AssetEvent::LoadedWithDependencies`].
fn handle_svg_loaded(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &Svg), Without<Sprite>>,
) {
    let loaded_ids = read_events!(svg_events, AssetEvent::LoadedWithDependencies);

    for (entity, svg) in &query {
        let id = svg.0.id();
        if loaded_ids.contains(&id) {
            if let Some(svg_file) = svg_assets.get(id) {
                let image_handle = images.add(svg_file.0.clone());
                commands
                    .entity(entity)
                    .insert(Sprite::from_image(image_handle));
                debug!("Added `Sprite` for `{id}` to entity {entity:?}");
            } else {
                warn!("`{id}` reported as loaded, but not found in `Assets`");
            }
        }
    }
}

/// Handles modified [`SvgFile`]s (e.g. through
/// [hot-reloading](https://github.com/bevyengine/bevy/blob/main/examples/asset/hot_asset_reloading.rs))
/// by updating existing [`Sprite`]s. This responds to [`AssetEvent::Modified`].
fn handle_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&Svg, &mut Sprite)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    for (svg, mut sprite) in &mut query {
        let id = svg.0.id();
        if modified_ids.contains(&id) {
            if let Some(svg_file) = svg_assets.get(id) {
                // Update the image data in-place instead of creating new handle
                if let Some(image) = images.get_mut(&sprite.image) {
                    *image = svg_file.0.clone();
                    debug!("Updated `Image` data for modified `{id}`");
                } else {
                    // Fallback: create new handle if old one is invalid
                    sprite.image = images.add(svg_file.0.clone());
                    debug!("Replaced `Handle<Image>` for modified `{id}`");
                }
            } else {
                warn!("`{id}` reported as modified, but not found in `Assets`");
            }
        }
    }
}

/// Handles removed and unused [`SvgFile`]s by cleaning up associated [`Sprite`]
/// components. This fires when an [`Asset`] is either explicitly removed from
/// the asset system, or removed due to the last strong handle being dropped.
/// This coresponds to [`AssetEvent::Removed`] and [`AssetEvent::Unused`],
/// respectively.
fn handle_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    query: Query<(Entity, &Svg), With<Sprite>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    for (entity, svg) in query {
        let id = svg.0.id();
        if removed_ids.contains(&id) {
            commands.entity(entity).remove::<Sprite>();
            info!("Removed `Sprite` for `{id}` from entity {entity:?}");
        }
    }
}
