use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn_scene(ui_panel());
}

fn ui_panel() -> impl Scene {
    bsn! {
        Node {
            width: px(128),
            height: px(128),
            border: UiRect::all(px(8)),
        }
        BorderColor::all(Color::Srgba(BLUE))
        Children [
            UiSvg("transparent.svg")
        ]
    }
}
