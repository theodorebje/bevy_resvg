use crate::{
    effects::components::{SvgColor, SvgSize},
    raster::{
        component::Svg,
        plugin::{
            macros::read_events,
            queue::{RasterIntent, RasterQueue},
            sizing::{raster_size, sprite_world},
        },
    },
    vector::asset::SvgAsset,
};
use bevy::prelude::*;

pub fn handle_svg_loaded(
    mut queue: ResMut<RasterQueue>,
    svg_assets: Res<Assets<SvgAsset>>,
    query: Query<(Entity, Ref<Svg>, Option<&SvgColor>, Option<&SvgSize>), Without<Sprite>>,
) {
    for (entity, svg, svg_color, svg_size) in &query {
        if queue.is_busy(entity) {
            continue;
        }
        let Some(vector) = svg_assets.get(svg.0.id()) else {
            continue;
        };
        let size = raster_size(vector, svg_size);

        queue.request(
            entity,
            vector,
            size,
            RasterIntent::InsertSprite {
                color: svg_color.copied().unwrap_or_default().0,
                world: sprite_world(vector),
            },
        );
        debug!(
            "Requested `Sprite` for `{}` to entity {entity:?}",
            svg.0.id()
        );
    }
}

pub fn handle_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgAsset>>,
    svg_assets: Res<Assets<SvgAsset>>,
    mut queue: ResMut<RasterQueue>,
    query: Query<(Entity, &Svg, Option<&SvgColor>, Option<&SvgSize>), With<Sprite>>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    for (entity, svg, svg_color, svg_size) in &query {
        let id = svg.0.id();
        if !modified_ids.contains(&id) {
            continue;
        }
        let Some(vector) = svg_assets.get(id) else {
            continue;
        };
        let size = raster_size(vector, svg_size);

        queue.request(
            entity,
            vector,
            size,
            RasterIntent::SwapSpriteImage {
                world: sprite_world(vector),
            },
        );
        if let Some(svg_color) = svg_color {
            info!("`Svg` asset {id:?} uses `SvgColor`: {:?}", svg_color.0);
        }
        if let Some(svg_size) = svg_size {
            info!("`Svg` asset {id:?} uses `SvgSize`: {:?}", svg_size);
        }
    }
}

pub fn handle_svg_removed(
    mut commands: Commands,
    mut queue: ResMut<RasterQueue>,
    mut svg_events: MessageReader<AssetEvent<SvgAsset>>,
    query: Query<(Entity, &Svg), With<Sprite>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    for (entity, svg) in query {
        let id = svg.0.id();
        if removed_ids.contains(&id) {
            queue.cancel(entity);
            commands.entity(entity).remove::<Sprite>();
            info!("Removed `Sprite` for `{id}` from entity {entity:?}");
        }
    }
}
