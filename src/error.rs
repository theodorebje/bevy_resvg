use resvg::usvg;
use thiserror::Error;

/// An error that occurs when loading, parsing, or rasterising a texture.
#[derive(Error, Debug)]
pub enum SvgError {
    #[error("could not read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse SVG: {0}")]
    Parse(#[from] usvg::Error),
    #[error("SVG must not have a zeroed viewBox")]
    Empty,
}

/// A type alias for brevity. Uses the [`SvgError`] enum as the error type.
///
/// See [`Result`](::std::result::Result) for details.
pub(crate) type Result<T> = ::std::result::Result<T, SvgError>;
