use bevy::prelude::*;
use bevy_pancam::{DirectionKeys, PanCam, PanCamPlugin};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin, PanCamPlugin))
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

fn setup(mut commands: Commands) {
    commands.spawn_scene(svg_scene());
}

fn svg_scene() -> impl Scene {
    bsn! {
        Svg("transparent.svg")
    }
}
