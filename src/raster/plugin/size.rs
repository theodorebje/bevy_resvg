use crate::{
    effects::components::SvgSize,
    raster::{
        component::{Svg, UiSvg},
        plugin::{
            queue::{RasterIntent, RasterQueue},
            sizing::{is_auto, raster_size, sprite_world},
        },
    },
    vector::asset::SvgAsset,
};
use bevy::prelude::*;

pub fn sync_svg_size_changes(
    svg_assets: Res<Assets<SvgAsset>>,
    mut queue: ResMut<RasterQueue>,
    svg_query: Query<(Entity, &Svg, &SvgSize, &Sprite), Changed<SvgSize>>,
    ui_svg_query: Query<(Entity, &UiSvg, &SvgSize, &ImageNode), Changed<SvgSize>>,
) {
    for (entity, svg, svg_size, _) in &svg_query {
        if is_auto(svg_size) {
            continue; // we don't have access to the camera in here
        }
        let Some(vector) = svg_assets.get(svg.0.id()) else {
            continue;
        };
        let size = raster_size(vector, Some(svg_size));

        queue.request(
            entity,
            vector,
            size,
            RasterIntent::SwapSpriteImage {
                world: sprite_world(vector),
            },
        );
    }

    for (entity, svg, svg_size, _) in &ui_svg_query {
        if is_auto(svg_size) {
            continue; // we don't have access to the camera in here
        }
        let Some(vector) = svg_assets.get(svg.0.id()) else {
            continue;
        };
        let size = raster_size(vector, Some(svg_size));

        queue.request(entity, vector, size, RasterIntent::SwapImageNodeImage);
    }
}

pub fn sync_removed_svg_size(
    svg_assets: Res<Assets<SvgAsset>>,
    mut queue: ResMut<RasterQueue>,
    mut removed_svg_sizes: RemovedComponents<SvgSize>,
    svg_query: Query<(Entity, &Svg, &Sprite), With<Svg>>,
    ui_svg_query: Query<(Entity, &UiSvg, &ImageNode), With<UiSvg>>,
) {
    for entity in removed_svg_sizes.read() {
        if let Ok((entity, svg, _)) = svg_query.get(entity)
            && let Some(vector) = svg_assets.get(svg.0.id())
        {
            queue.request(
                entity,
                vector,
                raster_size(vector, None),
                RasterIntent::SwapSpriteImage {
                    world: sprite_world(vector),
                },
            );
        }
        if let Ok((entity, svg, _)) = ui_svg_query.get(entity)
            && let Some(vector) = svg_assets.get(svg.0.id())
        {
            queue.request(
                entity,
                vector,
                raster_size(vector, None),
                RasterIntent::SwapImageNodeImage,
            );
        }
    }
}
