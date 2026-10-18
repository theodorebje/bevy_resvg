use crate::{
    effects::components::{SvgColor, SvgSize, Unit},
    raster::component::{Svg, UiSvg},
    vector::asset::SvgAsset,
};
use bevy::prelude::*;
use resvg::tiny_skia::IntSize;
use std::collections::HashSet;

/// The [`Plugin`] for initialising the
/// [Rasterised](https://en.wikipedia.org/wiki/Raster_graphics)
/// [`Asset`] and [`AssetLoader`](bevy::asset::AssetLoader).
pub struct SvgRasterPlugin;

impl Plugin for SvgRasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
                sync_svg_size_changes,
                sync_removed_svg_size,
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

/// Handles newly loaded [`SvgAsset`]s by adding [`Sprite`] components to waiting
/// entities.
fn handle_svg_loaded(
    mut commands: Commands,
    svg_assets: Res<Assets<SvgAsset>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, Ref<Svg>, Option<&SvgColor>, Option<&SvgSize>), Without<Sprite>>,
) {
    for (entity, svg, svg_color, svg_size) in &query {
        let id = svg.0.id();
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };

        commands.entity(entity).insert(Sprite {
            image: images.add(do_stuff(vector, svg_size)),
            color: svg_color.copied().unwrap_or_default().0,
            ..default()
        });
        debug!("Added `Sprite` for `{id}` to entity {entity:?}");
        if let Some(svg_color) = svg_color {
            info!("`Svg` entity {entity:?} has `SvgColor`: {:?}", svg_color.0);
        }
        if let Some(svg_size) = svg_size {
            info!("`Svg` entity {entity:?} has `SvgSize`: {:?}", svg_size);
        }
    }
}

fn do_stuff(vector: &SvgAsset, svg_size: Option<&SvgSize>) -> Image {
    fn compute_size(new: Unit, original: f32) -> u32 {
        match new {
            Unit::Auto => todo!(),
            Unit::Pixels(pixels) => pixels.get(),
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "already guaranteed to be positive and rounded"
            )]
            Unit::Scale(scale) => (original * scale.get()).round_ties_even() as u32,
        }
        .max(1)
    }

    let original_size = vector.0.size();
    let SvgSize { width, height } = svg_size.copied().unwrap_or_default();
    let computed_width = compute_size(width, original_size.width());
    let computed_height = compute_size(height, original_size.height());
    let int_size =
        IntSize::from_wh(computed_width, computed_height).unwrap_or_else(|| unreachable!());
    vector.render_to_image(int_size)
}

/// Handles modified [`SvgAsset`]s (e.g. through
/// [hot-reloading](https://github.com/bevyengine/bevy/blob/main/examples/asset/hot_asset_reloading.rs))
/// by updating existing [`Sprite`]s. This responds to [`AssetEvent::Modified`].
fn handle_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgAsset>>,
    svg_assets: Res<Assets<SvgAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&Svg, &mut Sprite, Option<&SvgColor>, Option<&SvgSize>)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    for (svg, mut sprite, svg_color, svg_size) in &mut query {
        let id = svg.0.id();
        if !modified_ids.contains(&id) {
            continue;
        }
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };

        sprite.image = images.add(do_stuff(vector, svg_size));

        if let Some(svg_color) = svg_color {
            info!("`Svg` asset {id:?} uses `SvgColor`: {:?}", svg_color.0);
        }
        if let Some(svg_size) = svg_size {
            info!("`Svg` asset {id:?} uses `SvgSize`: {:?}", svg_size);
        }
    }
}

/// Handles removed and unused [`SvgAsset`]s by cleaning up associated [`Sprite`]
/// components. This fires when an [`Asset`] is either explicitly removed from
/// the asset system, or removed due to the last strong handle being dropped.
/// This corresponds to [`AssetEvent::Removed`] and [`AssetEvent::Unused`],
/// respectively.
fn handle_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgAsset>>,
    query: Query<(Entity, &Svg, Option<&SvgColor>, Option<&SvgSize>), With<Sprite>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    for (entity, svg, svg_color, svg_size) in query {
        let id = svg.0.id();
        if !removed_ids.contains(&id) {
            continue;
        }

        commands.entity(entity).remove::<Sprite>();
        info!("Removed `Sprite` for `{id}` from entity {entity:?}");
        if let Some(svg_color) = svg_color {
            info!(
                "Removed `Svg` entity {entity:?} had `SvgColor`: {:?}",
                svg_color.0
            );
        }
        if let Some(svg_size) = svg_size {
            info!(
                "Removed `Svg` entity {entity:?} had `SvgSize`: {:?}",
                svg_size
            );
        }
    }
}

/// Handles newly loaded [`SvgAsset`]s by adding [`ImageNode`] components to
/// waiting UI entities.
fn handle_ui_svg_loaded(
    mut commands: Commands,
    svg_assets: Res<Assets<SvgAsset>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, Ref<UiSvg>, Option<&SvgColor>, Option<&SvgSize>), Without<ImageNode>>,
) {
    for (entity, svg, svg_color, svg_size) in &query {
        let id = svg.0.id();
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };

        commands.entity(entity).insert(ImageNode {
            image: images.add(do_stuff(vector, svg_size)),
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
        if let Some(svg_size) = svg_size {
            info!("`UiSvg` entity {entity:?} has `SvgSize`: {:?}", svg_size);
        }
    }
}

/// Handles modified [`SvgAsset`]s by updating existing [`ImageNode`]s.
/// This responds to [`AssetEvent::Modified`].
fn handle_ui_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgAsset>>,
    svg_assets: Res<Assets<SvgAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&UiSvg, &mut ImageNode, Option<&SvgColor>, Option<&SvgSize>)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    for (svg, mut sprite, svg_color, svg_size) in &mut query {
        let id = svg.0.id();
        if !modified_ids.contains(&id) {
            continue;
        }
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };

        sprite.image = images.add(do_stuff(vector, svg_size));

        if let Some(svg_color) = svg_color {
            info!("`UiSvg` asset {id:?} uses `SvgColor`: {:?}", svg_color.0);
        }
        if let Some(svg_size) = svg_size {
            info!("`UiSvg` asset {id:?} uses `SvgSize`: {:?}", svg_size);
        }
    }
}

/// Handles removed and unused [`SvgAsset`]s by cleaning up associated
/// [`ImageNode`] components in UI. This corresponds to
/// [`AssetEvent::Removed`] and [`AssetEvent::Unused`], respectively.
fn handle_ui_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgAsset>>,
    query: Query<(Entity, &UiSvg), With<ImageNode>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    for (entity, svg) in query {
        let id = svg.0.id();
        if !removed_ids.contains(&id) {
            continue;
        }
        commands.entity(entity).remove::<ImageNode>();
        info!("Removed `ImageNode` for `{id}` from entity {entity:?}");
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

/// Applies runtime [`SvgSize`] changes to already-inserted render components.
fn sync_svg_size_changes(
    svg_assets: Res<Assets<SvgAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut svg_query: Query<(&Svg, &SvgSize, &mut Sprite), Changed<SvgSize>>,
    mut ui_svg_query: Query<(&UiSvg, &SvgSize, &mut ImageNode), Changed<SvgSize>>,
) {
    for (svg, svg_size, mut sprite) in &mut svg_query {
        let id = svg.0.id();
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };

        sprite.image = images.add(do_stuff(vector, Some(svg_size)));
    }

    for (svg, svg_size, mut image_node) in &mut ui_svg_query {
        let id = svg.0.id();
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };

        image_node.image = images.add(do_stuff(vector, Some(svg_size)));
    }
}

/// Resets size to viewport when [`SvgSize`] is removed from an entity.
fn sync_removed_svg_size(
    svg_assets: Res<Assets<SvgAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut removed_svg_sizes: RemovedComponents<SvgSize>,
    mut svg_query: Query<(&Svg, &mut Sprite)>,
    mut ui_svg_query: Query<(&UiSvg, &mut ImageNode)>,
) {
    for entity in removed_svg_sizes.read() {
        if let Ok((svg, mut sprite)) = svg_query.get_mut(entity) {
            let id = svg.0.id();
            let Some(vector) = svg_assets.get(id) else {
                continue;
            };
            sprite.image = images.add(do_stuff(vector, None));
        }
        if let Ok((svg, mut image_node)) = ui_svg_query.get_mut(entity) {
            let id = svg.0.id();
            let Some(vector) = svg_assets.get(id) else {
                continue;
            };
            image_node.image = images.add(do_stuff(vector, None));
        }
    }
}
