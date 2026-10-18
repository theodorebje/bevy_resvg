use bevy::{
    camera_controller::pan_camera::{PanCamera, PanCameraPlugin},
    prelude::*,
};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin, PanCameraPlugin))
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

fn setup(mut commands: Commands) {
    commands.spawn_scene(svg_scene());
}

fn svg_scene() -> impl Scene {
    bsn! {
        Svg("transparent.svg")
        SvgSize { // texture resolution, does not change rendered aspect ratio
            width: Unit::pixels(1024),
            height: Unit::pixels(512),
        }
        // world size, does not change rasterised aspect ratio
        Transform::from_scale(Vec3::new(2.0, 1.0, 1.0))
    }
}
