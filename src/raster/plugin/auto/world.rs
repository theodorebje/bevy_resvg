use crate::{
    raster::{
        component::Svg,
        plugin::{
            auto::{AutoRaster, pixels_per_world_unit, quantise},
            queue::{RasterIntent, RasterQueue},
            sizing::{ONE, compute_size, is_auto, sprite_world},
        },
    },
    vector::asset::{IntSize, SvgAsset},
};
use bevy::{camera::visibility::ViewVisibility, prelude::*};
use resvg::usvg::NonZeroPositiveF32;
use std::num::NonZeroU32;

pub fn sync_svg_auto_sizes(
    cameras: Query<(&Camera, &GlobalTransform)>,
    svg_assets: Res<Assets<SvgAsset>>,
    images: Res<Assets<Image>>,
    mut queue: ResMut<RasterQueue>,
    mut commands: Commands,
    auto_rasters: Query<&AutoRaster>,
    mut sprites: Query<(
        Entity,
        &Svg,
        Option<&crate::effects::components::SvgSize>,
        &mut Sprite,
        &ViewVisibility,
    )>,
) {
    let Some((camera, camera_transform)) = cameras.iter().next() else {
        debug!("no camera found");
        return;
    };
    let Some(raw_pixels_per_unit) = pixels_per_world_unit(camera, camera_transform) else {
        debug!("world_to_viewport failed");
        return;
    };
    let raw_pixels_per_unit = raw_pixels_per_unit * camera.target_scaling_factor().unwrap_or(1.0);
    let pixels_per_unit = quantise(raw_pixels_per_unit);
    debug!("pixels_per_unit = {pixels_per_unit:?}");

    for (entity, svg, svg_size, mut sprite, view_visibility) in &mut sprites {
        if !view_visibility.get() {
            commands.entity(entity).remove::<AutoRaster>();
            continue;
        }
        let Some(svg_size) = svg_size.filter(|size| is_auto(size)) else {
            commands.entity(entity).remove::<AutoRaster>();
            continue;
        };
        let Some(vector) = svg_assets.get(svg.0.id()) else {
            continue;
        };

        let intrinsic = vector.0.size();
        let world = sprite_world(vector);
        let ppu_x = NonZeroPositiveF32::new(pixels_per_unit.x)
            .unwrap_or_else(|| NonZeroPositiveF32::new(1.0).unwrap());
        let ppu_y = NonZeroPositiveF32::new(pixels_per_unit.y)
            .unwrap_or_else(|| NonZeroPositiveF32::new(1.0).unwrap());
        let target = IntSize {
            width: compute_size(svg_size.width, intrinsic.width(), Some(ppu_x)),
            height: compute_size(svg_size.height, intrinsic.height(), Some(ppu_y)),
        };

        let applied_seed_raw = images
            .get(sprite.image.id())
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
        sprite.custom_size = Some(world);

        let covers = |size: IntSize| size.width >= target.width && size.height >= target.height;
        if covers(applied) {
            continue;
        }

        if queue.in_flight(entity).is_some_and(covers) {
            continue;
        }

        debug!("`Unit::Auto` wants {target:?}, applied = {:?}", applied);
        queue.request(
            entity,
            vector,
            target,
            RasterIntent::SwapSpriteImage {
                world: sprite_world(vector),
            },
        );
    }
}
