use bevy::{
    color::palettes::css::BLUE,
    prelude::*,
};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, tween_hue_over_time)
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
            SvgColor(Color::hsl(0.0, 1.0, 0.5))
        ]
    }
}

fn tween_hue_over_time(time: Res<Time>, mut svg_colors: Query<&mut SvgColor, With<UiSvg>>) {
    let hue = (time.elapsed_secs() * 60.0) % 360.0;
    let color = Color::hsl(hue, 1.0, 0.5);

    for mut svg_color in &mut svg_colors {
        svg_color.0 = color;
    }
}
