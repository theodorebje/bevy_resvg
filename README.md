# Bevy Resvg

A library for rendering SVGs in Bevy using the amazing
[Resvg](https://github.com/linebender/resvg) library.

[![Crates.io](https://img.shields.io/crates/v/bevy_resvg?style=for-the-badge&logo=rust)](https://crates.io/crates/bevy_resvg)
[![Documentation](https://img.shields.io/docsrs/bevy_resvg/latest?style=for-the-badge&logo=docsdotrs)](https://docs.rs/bevy_resvg)
[![Bevy Version](https://img.shields.io/badge/Bevy-v0.20-9B22FF?style=for-the-badge&logo=bevy)](https://bevy.org/news/bevy-0-20/)
[![Dependency Status](https://img.shields.io/deps-rs/bevy_resvg/latest?style=for-the-badge)](https://deps.rs/repo/github/theodorebje/bevy_resvg)

## Why not [`bevy_svg`](https://github.com/Weasy666/bevy_svg)?

I originally set out to add colour tinting to `bevy_svg`, as proposed in
[`bevy_svg/#54`](https://github.com/Weasy666/bevy_svg/issues/54). Supporting
that (along with gradients and CSS injection) turned out to need a different
rendering approach: rasterising SVGs with
[Resvg](https://github.com/linebender/resvg) rather than tessellating them into
meshes. That was too large a change to propose as a patch, so I started a
separate library instead. Thanks to [Weasy666](https://github.com/Weasy666/) for
`bevy_svg`, which is still a great choice if you need 3D rendering or heavy
zooming.

Funnily enough, I'm actually publishing this crate onto
[crates.io](https://crates.io/) on `bevy_svg`'s 5-year anniversary
([January 24th](https://github.com/Weasy666/bevy_svg/commit/b4d9041))!

**Bevy Resvg** takes a completely different approach to rendering SVGs compared
to `bevy_svg`. Instead of tessellating the SVG into a mesh, it first renders the
SVG as a raster image using the [Resvg](https://github.com/linebender/resvg)
library.

## Why not [Bevy Vello](https://github.com/linebender/bevy_vello) then?

I actually only found out about Bevy Vello after publishing this crate. For
those unaware (like I was earlier!), Bevy Vello is a Bevy plugin that can render
numerous types of vector graphics, from text to SVGs to even
[Lotties](https://lottie.github.io/) using the
[Vello](https://github.com/linebender/vello) crate. Vello focuses on GPU
compute, making it very quick. Thanks to this performance, it can perform
Just-in-Time (JIT) rasterisation of the vector graphics, allowing for crisp
graphics at any resolution.

However, Vello doesn't support as much of the Static SVG Spec as Resvg does. If
you need to load complex SVGs, Bevy Resvg is likely the best fit.

Here's a complete comparison between the three:

## Comparison between Bevy Resvg, `bevy_svg`, and Bevy Vello

|          Feature          |    |              Bevy Resvg               |    |     `bevy_svg`    |    |            Bevy Vello             |
| ------------------------- | -- | ------------------------------------- | -- | ----------------- | -- | --------------------------------- |
| Source Lines of Code[^1]  | 🔼 |                                  1707 | 🔽 |          1144[^2] | ⏫ |                          4537[^3] |
| Code Complexity[^1]       | 😐 |                                   128 | 😀 |           106[^2] | 😵 |                           370[^3] |
| Hot-reloading of SVGs     | ✅ | Supported                             | ✅ | Supported         | ❌ | Unsupported                       |
| Changing runtime colour   | ✅ | Supported                             | ❌ | Unsupported       | ❌ | Unsupported                       |
| Custom CSS injection      | ✅ | Supported                             | ❌ | Unsupported       | ❌ | Unsupported                       |
| Gradients                 | ✅ | Supported                             | ❌ | Unsupported       | ⚠️ | Inaccurate                        |
| Semi-transparency         | ✅ | Supported                             | ❌ | Unsupported       | ✅ | Supported                         |
| Positioning and sizing    | ✅ | Native (`Sprite` & `ImageNode`-based) | ⚠️ | Imprecise         | ✅ | Native (`Image`-based)            |
| Static SVG Spec Support   | ✅ | Fully Supported                       | ⚠️ | Partial Support   | ⚠️ | Partial Support                   |
| Rendered quality (normal) | ✅ | Crisp                                 | ✅ | Crisp             | ✅ | Crisp                             |
| Rendered quality (zoomed) | ⚠️ | Blurry (once) / Crisp (adaptive)      | ✅ | Crisp             | ✅ | Crisp                             |
| 3D-Rendering              | ❌ | Unsupported                           | ✅ | Supported         | ✅ | Supported                         |
| Animated SVGs             | ❌ | Unsupported                           | ❌ | Unsupported       | ❌ | Unsupported                       |
| Approach                  | 🖼️ | Rasterisation (once or adaptive)      | 🔺 | Tessellation      | ⚙️ | Rasterisation (JIT every frame)   |
| Output                    | 🏃‍➡️ | `Sprite` (2D) or `ImageNode` (UI)     | 🕸️ | `Mesh2d`          | 🔀 | `Mesh2d` with image-based texture |
| Licence                   | 🟰 | MIT OR Apache-2.0                     | 🟰 | MIT OR Apache-2.0 | 🟰 | MIT OR Apache-2.0                 |

>[!NOTE]
>In order to be able to hot-reload your SVGs, you must enable the
>`file_watcher` **BEVY FEATURE** (*not* a feature available in this crate).
>
>As a side note, you probably want to enable entire `dev` collection (again,
>the *Bevy feature*, there is *no* feature called `dev` in this crate). It comes
>with a handful of goods, including debug logs and
>[dev tools](https://docs.rs/bevy/latest/bevy/dev_tools/index.html).

…to be expanded…

## Okay, but when should I *actually* use Bevy Resvg over the others?

This is a very complex question to answer. The three projects overlap a lot, but
their trade-offs differ. Your best bet is probably going to be to just add one
to your list of dependencies and try it out.

If you're not up for waiting for 3 separate crates to compile, and you want some
more concrete examples than the
[comparison table above](#comparison-between-bevy-resvg-bevy_svg-and-bevy-vello),
then check out the longer explanations below.

### Bevy Resvg vs `bevy_svg`

Although I am very proud of this crate (it's my first ever library to be
published on [crates.io](https://crates.io/))!, I do realise that there are
situations in which `bevy_svg` makes more sense.

For starters, Bevy Resvg is a very young and immature project. If
you are looking for something more mature and battle-tested, you should probably
use `bevy_svg`.

Another feature missing from Bevy Resvg is 3D rendering, which has first-class
support in `bevy_svg`. Unfortunately, you can't easily render to 3D objects with
Bevy Resvg, yet. Don't worry, it's on the [To-do list](#todos)! In the meantime,
check out `bevy_svg` if you need to render to a 3D object.

Furthermore, if your game is dependent on zooming into the SVGs, `bevy_svg`
might fit your needs better. By default, Bevy Resvg only performs rasterisation
once, mimicking the behaviour of e.g. the
[Godot Engine](https://forum.godotengine.org/t/how-to-make-svg-not-pixelated/92365).
This has the unfortunate side effect of causing blurry images when zooming in.
However, `SvgSize::auto()` enables adaptive (async, on-demand) rasterisation,
which re-rasterises at the resolution the SVG currently occupies on screen, so
zoomed-in SVGs stay crisp at the cost of some extra GPU memory. `bevy_svg`,
however, tessellates the SVGs into a crisp `Mesh2d`, which results in
sharper rendering when zoomed in.

If you wish to mitigate the blur, you can set a custom target render size using
the `SvgSize` component. This allows you to render a larger image up front.
However, this has the caveat of requiring you to manually determine how far a
user is likely to zoom in on your SVGs. See the
[`custom_size`](./examples/custom_size.rs) example for a demonstration of this.

However, if you are in need of rendering semi-transparent SVGs, then Bevy Resvg
is one of only two options (to my knowledge), alongside Bevy Vello. Perhaps you
even want to change the runtime colour of your loaded SVGs. In that case, Bevy
Resvg supports it directly! Just modify the outputted
[Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)'s `color`
field and you've got yourself a tinted SVG! That's not possible (again, to my
knowledge) in `bevy_svg`. What about gradients? No problem! Resvg handles
gradients naturally too.

I'm not sure why (and, like many other things in this project, I haven't
bothered to test why), but positioning and especially sizing SVGs with
`bevy_svg` as children of sprites required hard-coded values back when I used
`bevy_svg`. I have yet to encounter that issue with Bevy Resvg.

Also, although I have no data to back this up, I would assume that Bevy Resvg
might be a tiny bit faster than `bevy_svg`. Meshes *feel* more expensive than
textures to me, however I am no expert in this area.

During testing, I found that `bevy_svg` couldn't load complex Inkscape-generated
SVGs, while Bevy Resvg handled them flawlessly just like any other SVG file.
Although you should not ship Inkscape-generated SVGs in your released game, it
can be beneficial to not have to export your SVGs as standard SVGs from Inkscape
each time you want to test a new design.

### When to use Bevy Resvg over Bevy Vello

To be written…

<!-- TODO: Write this section -->

## Usage

See the [`examples`](./examples/) directory for examples of how to use the Bevy
Resvg.

Currently, the only examples are:

- [`auto_size`](./examples/auto_size.rs): shows how to use `SvgSize::auto()` to
  automatically set an appropriate size.
- [`auto_size_ui`](./examples/auto_size_ui.rs): shows how to use
  `SvgSize::auto()` for changing the size of a scale-tweening UI element.
- [`color_change`](./examples/color_change.rs): shows how to update the
  colour tint of an `Svg` when an event (spacebar pressed) occurs
- [`color_hue_tween`](./examples/color_hue_tween.rs): showcase of an `Svg`
  that continuously tweens through all hues
- [`color_ui_change`](./examples/color_ui_change.rs): shows how to update the
  colour tint of a `UiSvg` when an event (spacebar pressed) occurs
- [`color_ui_hue_tween`](./examples/color_ui_hue_tween.rs): showcase of a
  `UiSvg` that continuously tweens through all hues
- [`color_ui`](./examples/color_ui.rs): shows how to apply a static colour
  tint to a `UiSvg`
- [`color`](./examples/color.rs): shows how to apply a static colour tint to
  an `Svg`
- [`crisp_shapes`](./examples/crisp_shapes.rs): shows how to
  customise `usvg::Options` to render SVGs with crisp edges and with
  anti-aliasing turned off.
- [`custom_aspect_ratio`](./examples/custom_aspect_ratio.rs): shows how to
  render an SVG to a custom target size with a non-native aspect ratio
- [`custom_size`](./examples/custom_size.rs): shows how to render an SVG to a
  custom target size using `SvgSize`
- [`custom_style_sheet`](./examples/custom_style_sheet.rs): shows how to
  customise `usvg::Options` while loading an SVG by setting `style_sheet`
- [`simple`](./examples/simple.rs): shows the most basic usage of Bevy Resvg
- [`simple_scene`](./examples/simple_scene.rs): shows basic `bsn!` and
  `World::spawn_scene` integration in Bevy Resvg
- [`ui`](./examples/ui.rs): shows how to render `UiSvg`s in UI nodes
- [`zoom`](./examples/zoom.rs): shows what happens when you zoom too far into
  an SVG

More examples are planned!

If you're too lazy to click the link, here are the contents of `simple.rs`:

```rust
use bevy::prelude::*;
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg: Handle<SvgAsset> = asset_server.load("transparent.svg");
    commands.spawn(Camera2d);
    commands.spawn(Svg(svg));
}
```

In real code, you most likely want to use scenes instead. Here are the contents
of `simple_scene.rs`:

```rust
use bevy::prelude::*;
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn_scene(svg_scene());
}

fn svg_scene() -> impl Scene {
    bsn! {
        Svg("transparent.svg")
    }
}
```

### Migration guide from `bevy_svg`

If you wish to migrate from `bevy_svg` to Bevy Resvg, you must do the following
actions:

>[!IMPORTANT]
>3D rendering is not yet supported in Bevy Resvg.

1. Run `cargo remove bevy_svg` and `cargo add bevy_resvg`.
2. If you were just using `bevy_svg::prelude` in your code, simply replace it
   with `bevy_resvg::prelude`.
3. Replace all occurrences of `Svg` with `SvgAsset` (make sure you're doing
   whole-word replacing. You don't want an `SvgAssetPlugin`!).
4. Replace all occurrences of `Svg2d` with either `Svg` or `UiSvg` (depending on
   how you're using it). This step requires a bit of care so that you don't use
   the wrong one!

## Todos

- [ ] 3D-Rendering support
- [x] Adaptive (on-demand) rasterisation
- [ ] Add more examples
- [ ] Add tests
- [x] Custom rendering size targets (not dependent on `viewBox` value) <a id="custom-render-size"></a>
- [x] UI rendering
- [ ] Animated SVGs!
- [ ] Expand [comparison table](#comparison-between-bevy-resvg-bevy_svg-and-bevy-vello)
  - [ ] Particularly, add performance comparisons
  - [ ] Benchmark adaptive rasterisation
- [x] Handle more
  [`AssetEvent`](https://docs.rs/bevy/latest/bevy/asset/enum.AssetEvent.html)s
  - [x] `Added`
  - [x] `Modified`
  - [x] `Removed`
  - [x] `Unused`
  - [x] `LoadedWithDependencies`
- [x] [`usvg::Options`](https://docs.rs/usvg/latest/usvg/struct.Options.html)
  support [^4]
  - [x] CSS support

## Minimum supported Rust version

Bevy Resvg's MSRV is 1.95 due to Bevy's MSRV.

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## Licence

[MIT](./LICENCE-MIT) OR [Apache-2.0](./LICENCE-APACHE), at your option.

[^1]: Calculated using [`scc`](https://github.com/boyter/scc), only counting Rust
code in the src directory.
[^2]: Based on commit
[`c1b0e29` in `Weasy666/bevy_svg`](https://github.com/Weasy666/bevy_svg/commit/c1b0e291368562f9aa57c9c9db67cfdf0e205bd8)
[^3]: Based on commit
[`659b06f` in `linebender/bevy_vello`](https://github.com/linebender/bevy_vello/commit/659b06fdb422dcd9992446d2c30cda024659b9f4)
[^4]: The advanced `image_href_resolver`, `font_resolver`, and `fontdb` settings
are not available due to missing serde support in `usvg`. If you require
changing these settings, please use the `resvg-option` branch. It's
unfortunately not released on [crates.io](https://crates.io) due to it
[`[patch]`ing](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section)
`usvg`, which is
[not allowed on crates.io](https://github.com/rust-lang/cargo/issues/13222).
Please follow the instructions in
[`resvg-option/USAGE.md`](https://github.com/theodorebje/bevy_resvg/blob/resvg-option/USAGE.md)
to set up `resvg-option`.
