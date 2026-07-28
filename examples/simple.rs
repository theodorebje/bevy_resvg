use bevy::prelude::*;
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn_scene(svg_scene());
}

fn svg_scene() -> impl Scene {
    bsn! {
        Svg("transparent.svg")
    }
}
