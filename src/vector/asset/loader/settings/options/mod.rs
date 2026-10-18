mod local;
mod macros;

use local::{ImageRenderingDef, ShapeRenderingDef, SizeDef, TextRenderingDef};
use macros::{enum_def, options_def};
use resvg::usvg::{ImageRendering, Options, ShapeRendering, Size, TextRendering};
use serde::{Deserialize, Serialize};

options_def! {
    /// A subset of [`Options`] that implements [`Serialize`] and [`Deserialize`]
    #[non_exhaustive]
    pub struct OptionsDef -> Options<'_> {
        /// Directory that will be used during relative paths resolving.
        ///
        /// Expected to be the same as the directory that contains the SVG file,
        /// but can be set to any.
        ///
        /// Default: `None`
        pub resources_dir: Option<std::path::PathBuf>,

        /// Target DPI.
        ///
        /// Impacts units conversion.
        ///
        /// Default: 96.0
        pub dpi: f32,

        /// A default font family.
        ///
        /// Will be used when no `font-family` attribute is set in the SVG.
        ///
        /// Default: Times New Roman
        pub font_family: String,

        /// A default font size.
        ///
        /// Will be used when no `font-size` attribute is set in the SVG.
        ///
        /// Default: 12
        pub font_size: f32,

        /// A list of languages.
        ///
        /// Will be used to resolve a `systemLanguage` conditional attribute.
        ///
        /// Format: en, en-US.
        ///
        /// Default: `[en]`
        pub languages: Vec<String>,

        /// Specifies the default shape rendering method.
        ///
        /// Will be used when an SVG element's `shape-rendering` property is set to `auto`.
        ///
        /// Default: `GeometricPrecision`
        #[serde(with = "ShapeRenderingDef")]
        pub shape_rendering: ShapeRendering,

        /// Specifies the default text rendering method.
        ///
        /// Will be used when an SVG element's `text-rendering` property is set to `auto`.
        ///
        /// Default: `OptimizeLegibility`
        #[serde(with = "TextRenderingDef")]
        pub text_rendering: TextRendering,

        /// Specifies the default image rendering method.
        ///
        /// Will be used when an SVG element's `image-rendering` property is set to `auto`.
        ///
        /// Default: `OptimizeQuality`
        #[serde(with = "ImageRenderingDef")]
        pub image_rendering: ImageRendering,

        /// Default viewport size to assume if there is no `viewBox` attribute and
        /// the `width` or `height` attributes are relative.
        ///
        /// Default: `(100., 100.)`
        #[serde(with = "SizeDef")]
        pub default_size: Size,

        /// A CSS stylesheet that should be injected into the SVG. Can be used to overwrite
        /// certain attributes.
        pub style_sheet: Option<String>,
    }
}
