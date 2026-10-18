/// The [`Component`](bevy::ecs::component::Component) that one needs to wrap
/// [`SvgAsset`](crate::vector::asset::SvgAsset)s in before spawning
/// them.
pub mod component;
/// The [`Plugin`](bevy::app::Plugin) for initialising the
/// [Rasterised](https://en.wikipedia.org/wiki/Raster_graphics)
/// [`Asset`](bevy::asset::Asset) and [`AssetLoader`](bevy::asset::AssetLoader).
pub(crate) mod plugin;
