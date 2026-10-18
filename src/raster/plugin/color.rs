use crate::{
    effects::components::SvgColor,
    raster::component::{Svg, UiSvg},
};
use bevy::prelude::*;

/// Applies runtime [`SvgColor`] changes to already-inserted render components.
pub fn sync_svg_color_changes(
    mut svg_query: Query<(&SvgColor, &mut Sprite), (With<Svg>, Changed<SvgColor>)>,
    mut ui_svg_query: Query<(&SvgColor, &mut ImageNode), (With<UiSvg>, Changed<SvgColor>)>,
) {
    for (svg_color, mut sprite) in &mut svg_query {
        sprite.color = svg_color.0;
    }

    for (svg_color, mut image_node) in &mut ui_svg_query {
        image_node.color = svg_color.0;
    }
}

/// Resets tint when [`SvgColor`] is removed from an entity.
pub fn sync_removed_svg_color(
    mut removed_svg_colors: RemovedComponents<SvgColor>,
    mut svg_query: Query<&mut Sprite, With<Svg>>,
    mut ui_svg_query: Query<&mut ImageNode, With<UiSvg>>,
) {
    for entity in removed_svg_colors.read() {
        if let Ok(mut sprite) = svg_query.get_mut(entity) {
            sprite.color = default();
        }
        if let Ok(mut image_node) = ui_svg_query.get_mut(entity) {
            image_node.color = default();
        }
    }
}
