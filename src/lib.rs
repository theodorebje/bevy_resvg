#![doc = include_str!("../README.md")]

/// Error utilities for this crate.
pub mod error;
/// The [`Plugin`](bevy::app::Plugin) for initialising the Bevy logic an
/// configuration provided by this crate.
pub mod plugin;
/// Tools and helpers for loading, rastering, and rendering
/// [`SVG`](https://en.wikipedia.org/wiki/SVG) files.
pub mod raster;
/// Tools and helpers for loading [`SVG`](https://en.wikipedia.org/wiki/SVG)
/// files.
pub(crate) mod vector;
