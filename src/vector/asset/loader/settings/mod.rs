/// Types used in [`OptionsDef`]. These are a [Serializable](Serialize) subset
/// of the types used in [`Options`](resvg::usvg::Options).
pub mod options;

use self::options::OptionsDef;
use serde::{Deserialize, Serialize};

/// Settings for the parsing and rendering of an SVG file.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct SvgAssetLoaderSettings {
    /// A [Serializable](Serialize) subset of [`Options`](resvg::usvg::Options).
    pub options: OptionsDef,
}
