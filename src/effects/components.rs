use std::num::NonZeroU32;

// Parts taken from 生于斯's fork of this repository:
// https://github.com/shengyusi-SYS/bevy_svg_ui
use bevy::prelude::*;
use resvg::usvg::NonZeroPositiveF32;

/// In order to prevent memory exhaustion from heavily-zoomed-in
/// [`SVG`](https://en.wikipedia.org/wiki/SVG)s, a resolution limit is put in place.
pub const DEFAULT_MAX_AUTO_RESOLUTION: NonZeroU32 = NonZeroU32::new(4096).unwrap();

/// A colour tint to apply to an SVG when rendering.
/// This will be multiplied with the SVG's original colours.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SvgColor(pub Color);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub enum Unit {
    /// Automatically render the SVG as the perfect size based on the size of the SVG in the
    /// viewport.
    ///
    /// Please note that this can be expensive if the size of the SVG in the viewport updates often.
    Auto {
        max: NonZeroU32,
    },
    /// Explicit size, in pixels
    Pixels(NonZeroU32),
    // Scale the viewBox by the specified factor. `1.0` is equal to the viewBox.
    Scale(NonZeroPositiveF32),
}

impl Unit {
    /// ## Panics
    ///
    /// If `pixels == 0`
    pub const fn pixels(pixels: u32) -> Self {
        Self::Pixels(NonZeroU32::new(pixels).expect("pixels may not be zero"))
    }
    pub const fn auto() -> Self {
        Self::Auto {
            max: DEFAULT_MAX_AUTO_RESOLUTION,
        }
    }
    /// ## Panics
    ///
    /// If `max == 0`
    pub const fn auto_max(pixels: u32) -> Self {
        Self::Auto {
            max: NonZeroU32::new(pixels).expect("pixels may not be zero"),
        }
    }
    /// ## Panics
    ///
    /// `factor` must be a positive, finite number.
    pub const fn scale(factor: f32) -> Self {
        assert!(factor.is_finite(), "factor must be finite");
        assert!(factor > 0.0, "factor must be positive");
        Self::Scale(unsafe { NonZeroPositiveF32::new_unchecked(factor) })
    }
    pub const fn original() -> Self {
        unsafe { Self::Scale(NonZeroPositiveF32::new_unchecked(1.)) }
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::original()
    }
}

/// The size to render the SVG as.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub struct SvgSize {
    pub width: Unit,
    pub height: Unit,
}

impl SvgSize {
    pub const fn both(unit: Unit) -> Self {
        Self {
            width: unit,
            height: unit,
        }
    }
    pub const fn width(unit: Unit) -> Self {
        Self {
            width: unit,
            height: Unit::original(),
        }
    }
    pub const fn height(unit: Unit) -> Self {
        Self {
            width: Unit::original(),
            height: unit,
        }
    }
    pub const fn auto() -> Self {
        Self::both(Unit::auto())
    }
    pub const fn original() -> Self {
        Self::both(Unit::original())
    }
}

impl Default for SvgSize {
    fn default() -> Self {
        Self::original()
    }
}
