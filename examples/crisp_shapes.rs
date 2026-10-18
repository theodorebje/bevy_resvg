use bevy::{
    camera_controller::pan_camera::{PanCamera, PanCameraPlugin},
    prelude::*,
};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()), // Bevy anti-aliases sprites by default. This disables anti-aliasing.
            SvgPlugin,
            PanCameraPlugin,
        ))
        .add_systems(Startup, (setup, setup_camera))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PanCamera {
            pan_speed: 100.,
            ..default()
        },
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg: Handle<SvgAsset> = asset_server
        .load_builder()
        .with_settings(|settings: &mut SvgAssetLoaderSettings| {
            settings.options.shape_rendering = bevy_resvg::resvg::usvg::ShapeRendering::CrispEdges; // This tells `resvg` to not do any anti-aliasing.
        })
        .load("transparent.svg");
    commands.spawn(Svg(svg));
}
