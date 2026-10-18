use crate::vector::asset::{IntSize, rasterise};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone, Copy, Debug)]
pub enum RasterIntent {
    InsertSprite { color: Color, world: Vec2 },
    SwapSpriteImage { world: Vec2 },
    InsertImageNode(Color),
    SwapImageNodeImage,
}

pub struct PendingRaster {
    pub entity: Entity,
    pub size: IntSize,
    pub intent: RasterIntent,
    cancel: Arc<AtomicBool>,
    pub task: Task<Option<Image>>,
}

#[derive(Resource, Default)]
pub struct RasterQueue(pub Vec<PendingRaster>);

impl RasterQueue {
    pub fn request(
        &mut self,
        entity: Entity,
        vector: &crate::vector::asset::SvgAsset,
        size: IntSize,
        intent: RasterIntent,
    ) {
        self.cancel(entity);
        debug!(
            "SPAWN raster={size:?} cost={:.2}MiB",
            f64::from(size.width.get()) * f64::from(size.height.get()) * 4.0 / (1024.0 * 1024.0),
        );
        let cancel1 = Arc::new(AtomicBool::new(false));
        let tree = Arc::clone(&vector.0);
        let cancel = Arc::clone(&cancel1);
        let task =
            AsyncComputeTaskPool::get().spawn(async move { rasterise(&tree, size, &cancel1) });
        self.0.push(PendingRaster {
            entity,
            size,
            intent,
            cancel,
            task,
        });
    }

    pub fn in_flight(&self, entity: Entity) -> Option<IntSize> {
        self.0
            .iter()
            .find(|pending| pending.entity == entity)
            .map(|pending| pending.size)
    }

    pub fn cancel(&mut self, entity: Entity) {
        let mut flagged = 0;
        for pending in &self.0 {
            if pending.entity == entity {
                pending.cancel.store(true, Ordering::Relaxed);
                flagged += 1;
            }
        }
        if flagged > 0 {
            debug!("CANCEL {entity:?}: abandoned {flagged} in-flight raster(s)");
        }
        self.0.retain(|pending| pending.entity != entity);
    }

    pub fn is_busy(&self, entity: Entity) -> bool {
        self.0.iter().any(|pending| pending.entity == entity)
    }

    pub fn cancel_dead(&mut self, commands: &mut Commands) {
        let mut dead = Vec::new();
        for pending in &self.0 {
            if commands.get_entity(pending.entity).is_err() {
                dead.push(pending.entity);
            }
        }
        for entity in dead {
            self.cancel(entity);
        }
    }
}

pub fn cancel_rasters_for_dead_entities(mut queue: ResMut<RasterQueue>, mut commands: Commands) {
    queue.cancel_dead(&mut commands);
}
