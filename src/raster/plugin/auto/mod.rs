pub mod ui;
pub mod world;

use crate::vector::asset::IntSize;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AutoRaster {
    pub applied: IntSize,
}

fn pixels_per_world_unit(camera: &Camera, camera_transform: &GlobalTransform) -> Option<Vec2> {
    let origin = camera
        .world_to_viewport(camera_transform, Vec3::ZERO)
        .ok()?;
    let along_x = camera.world_to_viewport(camera_transform, Vec3::X).ok()?;
    let along_y = camera.world_to_viewport(camera_transform, Vec3::Y).ok()?;

    Some(Vec2::new(along_x.x - origin.x, along_y.y - origin.y).abs())
}

const QUANTISE_RATIO: f32 = 1.5;

fn quantise_axis(value: f32) -> f32 {
    if !value.is_finite() || value <= 0.0 {
        return 1.0;
    }
    QUANTISE_RATIO.powf(value.log(QUANTISE_RATIO).ceil())
}

fn quantise(pixels_per_unit: Vec2) -> Vec2 {
    let longest = pixels_per_unit.x.max(pixels_per_unit.y);
    if !longest.is_finite() || longest <= 0.0 {
        return Vec2::ONE;
    }
    let factor = quantise_axis(longest) / longest;
    Vec2::new(pixels_per_unit.x * factor, pixels_per_unit.y * factor)
}

pub fn cleanup_auto_rasters(
    mut commands: Commands,
    query: Query<Entity, (With<AutoRaster>, Without<Sprite>, Without<ImageNode>)>,
) {
    for entity in &query {
        commands.entity(entity).remove::<AutoRaster>();
    }
}
