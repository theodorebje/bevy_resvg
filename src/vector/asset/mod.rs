/// The [`AssetLoader`](bevy::asset::AssetLoader) for [`SvgAsset`]s.
pub mod loader;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use resvg::{
    render_node,
    tiny_skia::Pixmap,
    usvg::{Transform, Tree},
};
use std::{
    num::NonZeroU32,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

/// An [`Asset`] containing an [`SVG`](https://en.wikipedia.org/wiki/SVG) file
/// deserialised visually losslessly[^1] as a [`Tree`] container.
///
/// [^1]: Only visually lossless for the static SVG subset.
#[derive(TypePath, Asset)]
pub struct SvgAsset(pub Arc<Tree>);

#[derive(Debug, Clone, Copy)]
pub struct IntSize {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

impl IntSize {
    const fn dimensions(self) -> (u32, u32) {
        (self.width.get(), self.height.get())
    }
}

impl Default for IntSize {
    fn default() -> Self {
        Self {
            width: NonZeroU32::new(1).unwrap(),
            height: NonZeroU32::new(1).unwrap(),
        }
    }
}

/// Rasterises a [`Tree`] into an [`Image`] of exactly [`size`](IntSize) pixels.
///
/// _May_ return [`None`] if `cancelled` was set before the iteration finished.
///
/// ## Cancellation
///
/// Between every top-level [`SVG`](https://en.wikipedia.org/wiki/SVG)
/// [node](https://en.wikipedia.org/wiki/Node_(computer_science)), `cancelled` is checked, and if
/// it's [`true`](bool), further iteration is prevented and [`None`] is returned. However, due to
/// visibility in [`resvg`], this only works for top-level nodes. A deeply nested node may still
/// block even after `cancelled` has been set to [`AtomicBool::new(true)`](AtomicBool::new).
pub fn rasterise(tree: &Tree, size: IntSize, cancel: &AtomicBool) -> Option<Image> {
    let (width, height) = size.dimensions();
    let original_size = tree.size();
    let (original_width, original_height) = (original_size.width(), original_size.height());

    #[expect(
        clippy::cast_precision_loss,
        reason = "Irrelevant, we could just as well have used f16"
    )]
    let (sx, sy) = (
        width as f32 / original_width,
        height as f32 / original_height,
    );
    let transform = Transform::from_scale(sx, sy);

    let mut pixmap = Pixmap::new(width, height).unwrap_or_else(|| unreachable!());
    let mut pixmap_mut = pixmap.as_mut();

    for node in tree.root().children() {
        if cancel.load(Ordering::Relaxed) {
            return None;
        }
        let Some(bbox) = node.abs_layer_bounding_box() else {
            continue;
        };
        let node_transform = transform.pre_translate(bbox.x(), bbox.y());
        render_node(node, node_transform, &mut pixmap_mut);
    }

    let size = Extent3d {
        width: pixmap.width(),
        height: pixmap.height(),
        depth_or_array_layers: 1,
    };
    Some(Image::new(
        size,
        TextureDimension::D2,
        pixmap.take(),
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    ))
}
