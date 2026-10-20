use crate::raster::plugin::{
    auto::AutoRaster,
    queue::{RasterIntent, RasterQueue},
};
use bevy::{prelude::*, tasks::futures::check_ready};

pub fn apply_finished_rasters(
    mut queue: ResMut<RasterQueue>,
    mut images: ResMut<Assets<Image>>,
    mut auto: Query<&mut AutoRaster>,
    mut commands: Commands,
    mut sprites: Query<&mut Sprite>,
    mut nodes: Query<&mut ImageNode>,
) {
    let mut index = 0;
    while index < queue.0.len() {
        let Some(result) = check_ready(&mut queue.0[index].task) else {
            index += 1;
            continue;
        };
        let pending = queue.0.remove(index);

        let Some(image) = result else {
            debug!("Rasterisation cancelled for {:?}", pending.entity);
            continue;
        };

        let handle = images.add(image);
        debug!(
            "APPLY {:?} {:?} -> {:?}",
            pending.entity, pending.intent, pending.size
        );
        match pending.intent {
            RasterIntent::InsertSprite { color, world } => {
                commands.entity(pending.entity).try_insert(Sprite {
                    image: handle,
                    color,
                    custom_size: Some(world),
                    ..default()
                });
            }
            RasterIntent::SwapSpriteImage { world } => {
                if let Ok(mut sprite) = sprites.get_mut(pending.entity) {
                    sprite.image = handle;
                    sprite.custom_size = Some(world);
                }
            }
            RasterIntent::InsertImageNode(color) => {
                commands.entity(pending.entity).try_insert(ImageNode {
                    image: handle,
                    color,
                    ..default()
                });
            }
            RasterIntent::SwapImageNodeImage => {
                if let Ok(mut node) = nodes.get_mut(pending.entity) {
                    node.image = handle;
                }
            }
        }

        if let Ok(mut state) = auto.get_mut(pending.entity) {
            state.applied = pending.size;
        }
    }
}
