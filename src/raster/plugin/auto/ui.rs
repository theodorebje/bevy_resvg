use crate::{
    raster::{
        component::UiSvg,
        plugin::{
            auto::AutoRaster,
            queue::{RasterIntent, RasterQueue},
            sizing::{ONE, is_auto, known_size, round_f32},
        },
    },
    vector::asset::{IntSize, SvgAsset},
};
use bevy::{camera::visibility::ViewVisibility, prelude::*, ui::ComputedNode};
use std::num::NonZeroU32;

pub fn sync_ui_svg_auto_sizes(
    svg_assets: Res<Assets<SvgAsset>>,
    images: Res<Assets<Image>>,
    mut queue: ResMut<RasterQueue>,
    mut commands: Commands,
    auto_rasters: Query<&AutoRaster>,
    nodes: Query<
        (
            Entity,
            &UiSvg,
            &crate::effects::components::SvgSize,
            &ImageNode,
            &ComputedNode,
            &ViewVisibility,
        ),
        With<UiSvg>,
    >,
) {
    for (entity, svg, svg_size, image_node, computed, view_visibility) in &nodes {
        if !view_visibility.get() || !is_auto(svg_size) {
            commands.entity(entity).remove::<AutoRaster>();
            continue;
        }
        let Some(vector) = svg_assets.get(svg.0.id()) else {
            continue;
        };

        let node_size = computed.size;
        if !node_size.is_finite() || node_size.x < 1.0 || node_size.y < 1.0 {
            continue;
        }

        let intrinsic = vector.0.size();
        let [computed_width, computed_height] = computed
            .size
            .to_array()
            .map(|f| round_f32(f).unwrap_or(ONE));
        let width = known_size(svg_size.width, intrinsic.width(), computed_width);
        let height = known_size(svg_size.height, intrinsic.height(), computed_height);
        let target = IntSize { width, height };

        let applied_seed_raw = images
            .get(image_node.image.id())
            .map_or(UVec2::ZERO, Image::size);
        let applied_seed = IntSize {
            width: NonZeroU32::new(applied_seed_raw.x).unwrap_or(ONE),
            height: NonZeroU32::new(applied_seed_raw.y).unwrap_or(ONE),
        };
        let applied = auto_rasters.get(entity).map_or_else(
            |_| {
                commands.entity(entity).insert(AutoRaster {
                    applied: applied_seed,
                });
                applied_seed
            },
            |ar| ar.applied,
        );

        debug!(
            "UI raster={}x{} node={node_size:?}",
            target.width, target.height,
        );

        let covers = |size: IntSize| size.width >= target.width && size.height >= target.height;
        if covers(applied) || queue.in_flight(entity).is_some_and(covers) {
            continue;
        }

        queue.request(entity, vector, target, RasterIntent::SwapImageNodeImage);
    }
}
