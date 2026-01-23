use crate::raster::{
    asset::{SvgRasterAsset, loader::SvgRasterAssetLoader},
    component::SvgRasterComponent,
};
use bevy::prelude::*;

pub struct SvgRasterPlugin;

impl Plugin for SvgRasterPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgRasterAsset>()
            .init_asset_loader::<SvgRasterAssetLoader>()
            .add_systems(Update, spawn_svg_sprites);
    }
}

fn spawn_svg_sprites(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgRasterAsset>>,
    svg_assets: Res<Assets<SvgRasterAsset>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &SvgRasterComponent), Without<Sprite>>,
) {
    for event in svg_events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } => {
                // Find all entities waiting for this SVG
                for (entity, raster_component) in &query {
                    if raster_component.0.id() == *id
                        && let Some(svg_asset) = svg_assets.get(*id)
                    {
                        let handle = images.add(svg_asset.0.clone());

                        commands.entity(entity).insert(Sprite::from_image(handle));
                    }
                }
            }
            AssetEvent::Added { id } => {
                debug!("added SVG asset '{id}'");
            }
            other => {
                // TODO: Handle more events
                warn!("Unimplemented Event for SVG raster assets: '{other:#?}'");
            }
        }
    }
}
