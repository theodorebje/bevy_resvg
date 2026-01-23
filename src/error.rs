use resvg::usvg;
use thiserror::Error;

/// An error that occurs when loading a texture
#[derive(Error, Debug)]
pub enum SvgError {
    #[error("could not read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to load an SVG: {0}")]
    Svg(#[from] usvg::Error),
    #[error("SVG is empty")]
    Empty,
}

pub(crate) type Result<T> = ::std::result::Result<T, SvgError>;
