/// The [`Asset`](bevy::asset::Asset) and
/// [`AssetLoader`](bevy::asset::AssetLoader) for rasterising
/// [`SVG`](https://en.wikipedia.org/wiki/SVG) files.
pub mod asset;
/// The [`Component`](bevy::ecs::component::Component) that one needs to wrap
/// [`SvgRasterAsset`](crate::raster::asset::SvgRasterAsset)s in before spawning
/// them.
pub mod component;
/// The [`Plugin`](bevy::app::Plugin) for initialising the
/// [Rasterised](https://en.wikipedia.org/wiki/Raster_graphics)
/// [`Asset`](bevy::asset::Asset) and [`AssetLoader`](bevy::asset::AssetLoader).
pub(crate) mod plugin;
