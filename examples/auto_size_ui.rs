use std::ops::Mul;

use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_resvg::prelude::*;

const START_SIZE: f32 = 50.;
const GROWTH: f32 = 5.;
const SHRINK_AFTER: f32 = 1.65;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, tween_size_over_time)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn_scene(svg_panel());
}

fn svg_panel() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            UiSvg("transparent.svg")
            Node {
                border: UiRect::all(px(4)),
            }
            BorderColor::all(Color::Srgba(BLUE))
            SvgSize::auto()
        ]
    }
}

fn tween_size_over_time(time: Res<Time>, mut panels: Query<&mut Node, With<UiSvg>>) {
    let elapsed_time = time.elapsed_secs();
    let cycle = elapsed_time % (SHRINK_AFTER * 2.);
    let size = if cycle < SHRINK_AFTER {
        START_SIZE.mul(GROWTH.powf(cycle))
    } else {
        START_SIZE
            .mul(GROWTH.powf(SHRINK_AFTER))
            .mul(GROWTH.powf(-(cycle - SHRINK_AFTER)))
    };

    for mut node in &mut panels {
        node.width = Val::Px(size);
        node.height = Val::Px(size);
    }
}
