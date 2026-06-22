use crate::{
    effects::components::SvgColor,
    raster::{
        asset::{SvgFile, loader::SvgFileLoader},
        component::{Svg, UiSvg},
    },
};
use bevy::prelude::*;
use std::collections::HashSet;

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
                (
                    handle_svg_loaded,
                    handle_svg_modified,
                    handle_svg_removed,
                    handle_ui_svg_loaded,
                    handle_ui_svg_modified,
                    handle_ui_svg_removed,
                    sync_svg_color_changes,
                    sync_removed_svg_color,
                ),
            );
    }
}

macro_rules! opt_read_events {
    ($svg_events:expr, $($asset_event:path) | +) => {{
        let mut ids = None;
        for event in $svg_events.read() {
            match event {
                $($asset_event { id }) | + => {
                    ids.get_or_insert_with(HashSet::new).insert(*id);
                }
                _ => (),
            }
        }
        ids
    }};
}

macro_rules! read_events {
    ($svg_events:expr, $($asset_event:path) | +) => {{
        let Some(ids) = opt_read_events!($svg_events, $($asset_event) | +) else {
            return;
        };
        ids
    }};
}

fn new_image_from_svg(
    id: AssetId<SvgFile>,
    svg_assets: &Assets<SvgFile>,
    images: &mut Assets<Image>,
) -> Option<Handle<Image>> {
    svg_assets.get(id).map_or_else(
        || {
            warn!("`{id}` reported in events, but not found in `Assets`");
            None
        },
        |svg_file| Some(images.add(svg_file.0.clone())),
    )
}

fn sync_existing_image_from_svg(
    id: AssetId<SvgFile>,
    svg_assets: &Assets<SvgFile>,
    images: &mut Assets<Image>,
    image_handle: &mut Handle<Image>,
) {
    if let Some(svg_file) = svg_assets.get(id) {
        // `get_mut` yields `&mut Image` on 0.18 but a `Mut<Image>` on 0.19, which
        // needs a `mut` binding to assign through; that `mut` is unused on 0.18.
        #[allow(unused_mut)]
        if let Some(mut image) = images.get_mut(image_handle.id()) {
            *image = svg_file.0.clone();
            debug!("Updated `Image` data for modified `{id}`");
        } else {
            *image_handle = images.add(svg_file.0.clone());
            debug!("Replaced `Handle<Image>` for modified `{id}`");
        }
    } else {
        warn!("`{id}` reported as modified, but not found in `Assets`");
    }
}

/// Handles newly loaded [`SvgFile`]s by adding [`Sprite`] components to waiting
/// entities. This responds to [`AssetEvent::LoadedWithDependencies`] and newly
/// added [`Svg`] components whose assets are already loaded.
fn handle_svg_loaded(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, Ref<Svg>, Option<&SvgColor>), Without<Sprite>>,
) {
    let loaded_ids = opt_read_events!(svg_events, AssetEvent::LoadedWithDependencies);

    for (entity, svg, svg_color) in &query {
        let id = svg.0.id();
        if (svg.is_added() || loaded_ids.as_ref().is_some_and(|x| x.contains(&id)))
            && let Some(image_handle) = new_image_from_svg(id, &svg_assets, &mut images)
        {
            commands.entity(entity).insert(Sprite {
                image: image_handle,
                color: svg_color.copied().unwrap_or_default().0,
                ..default()
            });
            debug!("Added `Sprite` for `{id}` to entity {entity:?}");
            if let Some(svg_color) = svg_color {
                info!("`Svg` entity {entity:?} has `SvgColor`: {:?}", svg_color.0);
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
    mut query: Query<(&Svg, &mut Sprite, Option<&SvgColor>)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    for (svg, mut sprite, svg_color) in &mut query {
        let id = svg.0.id();
        if modified_ids.contains(&id) {
            sync_existing_image_from_svg(id, &svg_assets, &mut images, &mut sprite.image);
            if let Some(svg_color) = svg_color {
                info!("`Svg` asset `{id}` uses `SvgColor`: {:?}", svg_color.0);
            }
        }
    }
}

/// Handles removed and unused [`SvgFile`]s by cleaning up associated [`Sprite`]
/// components. This fires when an [`Asset`] is either explicitly removed from
/// the asset system, or removed due to the last strong handle being dropped.
/// This corresponds to [`AssetEvent::Removed`] and [`AssetEvent::Unused`],
/// respectively.
fn handle_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    query: Query<(Entity, &Svg, Option<&SvgColor>), With<Sprite>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    for (entity, svg, svg_color) in query {
        let id = svg.0.id();
        if removed_ids.contains(&id) {
            commands.entity(entity).remove::<Sprite>();
            info!("Removed `Sprite` for `{id}` from entity {entity:?}");
            if let Some(svg_color) = svg_color {
                info!(
                    "Removed `Svg` entity {entity:?} had `SvgColor`: {:?}",
                    svg_color.0
                );
            }
        }
    }
}

/// Handles newly loaded [`SvgFile`]s by adding [`ImageNode`] components to
/// waiting entities in UI. This responds to
/// [`AssetEvent::LoadedWithDependencies`] and newly added [`UiSvg`] components
/// whose assets are already loaded.
fn handle_ui_svg_loaded(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, Ref<UiSvg>, Option<&SvgColor>), Without<ImageNode>>,
) {
    let loaded_ids = opt_read_events!(svg_events, AssetEvent::LoadedWithDependencies);

    for (entity, svg, svg_color) in &query {
        let id = svg.0.id();
        if (svg.is_added() || loaded_ids.as_ref().is_some_and(|x| x.contains(&id)))
            && let Some(image_handle) = new_image_from_svg(id, &svg_assets, &mut images)
        {
            commands.entity(entity).insert(ImageNode {
                image: image_handle,
                color: svg_color.copied().unwrap_or_default().0,
                ..default()
            });
            debug!("Added `ImageNode` for `{id}` to entity {entity:?}");
            if let Some(svg_color) = svg_color {
                info!(
                    "`UiSvg` entity {entity:?} has `SvgColor`: {:?}",
                    svg_color.0
                );
            }
        }
    }
}

/// Handles modified [`SvgFile`]s by updating existing [`ImageNode`]s.
/// This responds to [`AssetEvent::Modified`].
fn handle_ui_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&UiSvg, &mut ImageNode, Option<&SvgColor>)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    for (svg, mut image_node, svg_color) in &mut query {
        let id = svg.0.id();
        if modified_ids.contains(&id) {
            sync_existing_image_from_svg(id, &svg_assets, &mut images, &mut image_node.image);
            if let Some(svg_color) = svg_color {
                info!("`UiSvg` asset `{id}` uses `SvgColor`: {:?}", svg_color.0);
            }
        }
    }
}

/// Handles removed and unused [`SvgFile`]s by cleaning up associated
/// [`ImageNode`] components in UI. This corresponds to
/// [`AssetEvent::Removed`] and [`AssetEvent::Unused`], respectively.
fn handle_ui_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    query: Query<(Entity, &UiSvg), With<ImageNode>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    for (entity, svg) in query {
        let id = svg.0.id();
        if removed_ids.contains(&id) {
            commands.entity(entity).remove::<ImageNode>();
            info!("Removed `ImageNode` for `{id}` from entity {entity:?}");
        }
    }
}

/// Applies runtime [`SvgColor`] changes to already-inserted render components.
fn sync_svg_color_changes(
    mut svg_query: Query<(&SvgColor, &mut Sprite), (With<Svg>, Changed<SvgColor>)>,
    mut ui_svg_query: Query<(&SvgColor, &mut ImageNode), (With<UiSvg>, Changed<SvgColor>)>,
) {
    for (svg_color, mut sprite) in &mut svg_query {
        sprite.color = svg_color.0;
    }

    for (svg_color, mut image_node) in &mut ui_svg_query {
        image_node.color = svg_color.0;
    }
}

/// Resets tint when [`SvgColor`] is removed from an entity.
fn sync_removed_svg_color(
    mut removed_svg_colors: RemovedComponents<SvgColor>,
    mut svg_query: Query<&mut Sprite, With<Svg>>,
    mut ui_svg_query: Query<&mut ImageNode, With<UiSvg>>,
) {
    for entity in removed_svg_colors.read() {
        if let Ok(mut sprite) = svg_query.get_mut(entity) {
            sprite.color = default();
        }
        if let Ok(mut image_node) = ui_svg_query.get_mut(entity) {
            image_node.color = default();
        }
    }
}
