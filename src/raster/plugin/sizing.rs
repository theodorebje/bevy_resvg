use crate::{
    effects::components::{SvgSize, Unit},
    vector::asset::{IntSize, SvgAsset},
};
use bevy::prelude::*;
use resvg::usvg::NonZeroPositiveF32;
use std::num::NonZeroU32;

pub const ONE: NonZeroU32 = NonZeroU32::new(1).unwrap();

pub const fn round_f32(f: f32) -> Option<NonZeroU32> {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "We are intentionally rounding and making it signless"
    )]
    NonZeroU32::new(f.round_ties_even() as u32)
}

const fn f32_mul(a: f32, b: Option<NonZeroPositiveF32>, max: Option<NonZeroU32>) -> NonZeroU32 {
    let x = match b {
        Some(b) => round_f32(a * b.get()),
        None => round_f32(a),
    };
    if let Some(x) = x {
        if let Some(max) = max {
            return if x.get() < max.get() { x } else { max };
        }
        return x;
    }
    ONE
}

pub const fn known_size(unit: Unit, intrinsic: f32, pixels: NonZeroU32) -> NonZeroU32 {
    match unit {
        Unit::Auto { max } => {
            if pixels.get() < max.get() {
                pixels
            } else {
                max
            }
        }
        Unit::Pixels(pixels) => pixels,
        Unit::Scale(scale) => f32_mul(intrinsic, Some(scale), None),
    }
}

pub const fn compute_size(
    unit: Unit,
    intrinsic: f32,
    pixels_per_unit: Option<NonZeroPositiveF32>,
) -> NonZeroU32 {
    match unit {
        Unit::Auto { max } => f32_mul(intrinsic, pixels_per_unit, Some(max)),
        Unit::Pixels(pixels) => pixels,
        Unit::Scale(scale) => f32_mul(intrinsic, Some(scale), None),
    }
}

pub const fn is_auto(svg_size: &SvgSize) -> bool {
    let SvgSize { width, height } = svg_size;
    matches!(
        (width, height),
        (_, Unit::Auto { max: _ }) | (Unit::Auto { max: _ }, _)
    )
}

pub fn raster_size(vector: &SvgAsset, svg_size: Option<&SvgSize>) -> IntSize {
    let intrinsic = vector.0.size();
    let SvgSize { width, height } = svg_size.copied().unwrap_or_default();
    IntSize {
        width: compute_size(width, intrinsic.width(), None),
        height: compute_size(height, intrinsic.height(), None),
    }
}

pub fn sprite_world(vector: &SvgAsset) -> Vec2 {
    let size = vector.0.size();
    Vec2::new(size.width(), size.height())
}
