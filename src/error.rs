use resvg::usvg;
use thiserror::Error;

/// An error that is returned when loading, parsing, or rasterising a texture.
#[derive(Error, Debug)]
pub enum SvgError {
    /// Occurs when the [`AssetLoader`](bevy::asset::AssetLoader)'s
    /// [`Reader`](bevy::asset::io::Reader) couldn't read any bytes for some
    /// reason.
    #[error("could not read file: {0}")]
    Io(#[from] std::io::Error),
    /// Occurs when trying to parse the SVG as a node tree, and an SVG invariant
    /// is violated.
    #[error("failed to parse SVG: {0}")]
    Parse(#[from] usvg::Error),
    /// Occurs when trying to rasterise the parsed SVG, if either the width or
    /// the height of the
    /// [`viewBox`](https://svgwg.org/svg2-draft/coords.html#ViewBoxAttribute)
    /// is zero.
    #[error("SVG must not have a zeroed viewBox")]
    Empty,
}

/// A type alias for brevity. Uses the [`SvgError`] enum as the error type.
///
/// See [`Result`](::std::result::Result) for details.
pub(crate) type Result<T> = ::std::result::Result<T, SvgError>;
