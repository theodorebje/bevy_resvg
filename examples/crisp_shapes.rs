use bevy::prelude::*;
use bevy_pancam::{DirectionKeys, PanCam, PanCamPlugin};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()), // Bevy anti-aliases sprites by default. This disables anti-aliasing.
            SvgPlugin,
            PanCamPlugin,
        ))
        .add_systems(Startup, (setup, setup_camera))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PanCam {
            grab_buttons: vec![],
            move_keys: DirectionKeys::NONE,
            zoom_to_cursor: false,
            ..default()
        },
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg: Handle<SvgFile> = asset_server.load_builder().with_settings(
        |settings: &mut SvgFileLoaderSettings| {
            settings.options.shape_rendering = bevy_resvg::resvg::usvg::ShapeRendering::CrispEdges; // This tells `resvg` to not do any anti-aliasing.
        },
    ).load("transparent.svg");
    commands.spawn(Svg(svg));
}
