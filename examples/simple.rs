use bevy::prelude::*;
use bevy_resvg::{asset::SvgAsset, component::SvgComponent, plugin::SvgPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg: Handle<SvgAsset> = asset_server.load("transparent.svg");
    commands.spawn(Camera2d);
    commands.spawn(SvgComponent(svg));
}
