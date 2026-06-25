use bevy::prelude::*;
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg: Handle<SvgFile> = asset_server
        .load_builder()
        .with_settings(|settings: &mut SvgFileLoaderSettings| {
            settings.options.style_sheet = Some(String::from(
                "circle {
                    fill: #00ff00;
                    stroke: #ff00ff;
                }",
            ));
        })
        .load("transparent.svg");
    commands.spawn(Camera2d);
    commands.spawn(Svg(svg));
}
