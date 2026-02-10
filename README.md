# Bevy Resvg

A simple library for rendering SVGs in Bevy using the amazing
[Resvg](https://github.com/linebender/resvg) library.

[![Crates.io](https://img.shields.io/crates/v/bevy_resvg?style=for-the-badge&logo=rust)](https://crates.io/crates/bevy_resvg)
[![Documentation](https://img.shields.io/docsrs/bevy_resvg/latest?style=for-the-badge&logo=docsdotrs)](https://docs.rs/bevy_resvg)
[![Bevy Version](https://img.shields.io/badge/Bevy-v0.18-9B22FF?style=for-the-badge&logo=bevy)](https://bevy.org/news/bevy-0-18/)
[![Dependency Status](https://img.shields.io/deps-rs/bevy_resvg/latest?style=for-the-badge)](https://deps.rs/repo/github/theodorebje/bevy_resvg)

## Why not [`bevy_svg`](https://github.com/Weasy666/bevy_svg)?

I originally tried forking `bevy_svg` to add colour tinting support to the SVGs,
as proposed by [`bevy_svg/#54`](https://github.com/Weasy666/bevy_svg/issues/54).
However, I quickly noticed that the `bevy_svg` had so much technical debt
(no offense to [Weasy666](https://github.com/Weasy666/), your library has
clearly worked for many, many people) due to its 5 years of existence that it
was just easier to start a new library from scratch.

Funnily enough, I'm actually publishing this crate onto
[crates.io](https://crates.io/) on `bevy_svg`'s 5-year anniversary
([January 24th](https://github.com/Weasy666/bevy_svg/commit/b4d9041))!

**Bevy Resvg** takes a completely different approach to rendering SVGs compared
to `bevy_svg`. Instead of tesselating the SVG into a mesh, it first renders the
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

However, Vello isn't perfect. Most importantly, it doesn't support as much of
the Static SVG Spec as Resvg does. If you need to load complex SVGs, Bevy Resvg
is simply going to be the best choice.

Here's a complete comparison between the three:

## Comparison between Bevy Resvg, `bevy_svg`, and Bevy Vello

|          Feature          |    |           Bevy Resvg           |    |     `bevy_svg`      |    |           Bevy Vello            |
| ------------------------- | -- | ------------------------------ | -- | ------------------- | -- | ------------------------------- |
| Source Lines of Code      | 🔽 |                        233[^1] | 🔼 |        1904[^1][^2] | ⏫ |                    6361[^1][^3] |
| Code Complexity           | 😀 |                         11[^1] | 😵‍💫 |         145[^1][^2] | 😵 |                     409[^1][^3] |
| Hot-reloading of SVGs     | ✅ | Supported                      | ✅ | Supported           | ❌ | Unsupported                     |
| Changing runtime color    | ✅ | Supported                      | ❌ | Unsupported         | ❌ | Unsupported                     |
| Gradients                 | ✅ | Supported                      | ❌ | Unsupported         | ⚠️ | Inaccurate                      |
| Semi-transparency         | ✅ | Supported                      | ❌ | Unsupported         | ✅ | Supported                       |
| Positioning and sizing    | ✅ | Native (`Sprite`-based)        | ❌ | Janky and imprecise | ✅ | Native (`Image`-based)          |
| Static SVG Spec Support   | ✅ | Fully Supported                | ⚠️ | Partial Support     | ⚠️ | Partial Support                 |
| Rendered quality (normal) | ✅ | Crisp                          | ✅ | Crisp               | ✅ | Crisp                           |
| Rendered quality (zoomed) | ❌ | Blurry and Pixelated           | ✅ | Crisp               | ✅ | Crisp                           |
| 3D-Rendering              | ❌ | Unsupported                    | ✅ | Supported           | ✅ | Supported                       |
| Animated SVGs             | ❌ | Unsupported                    | ❌ | Unsupported         | ❌ | Unsupported                     |
| Approach                  | 🖼️ | Rasterisation (once)           | 🔺 | Tesselation         | ⚙️ | Rasterisation (JIT every frame) |
| Output                    | 🏃‍➡️ | Sprite                         | 🕸  | Mesh2d              | 🔀 | Mesh2d with image-based texture |
| Licence                   | 🟰 | MIT OR Apache-2.0              | 🟰 | MIT OR Apache-2.0   | 🟰 | MIT OR Apache-2.0               |

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

This is a very complex question to answer. Your best bet is probably going to be
to just add one to your list of dependencies and try it out; all three projects
are quite interchangeable with each other.

If you're not up for waiting for 3 separate crates to compile, and you want some
more concrete examples than the
[comparison table above](#comparison-between-bevy-resvg-bevy_svg-and-bevy-vello),
then check out some of the longer explanations below.

### When to use Bevy Resvg over `bevy_svg`

Although I am very proud of this small little crate (it's my first ever library
to be published on [crates.io](https://crates.io/)!, I do realise that there are situations in
which `bevy_svg` simply makes more sense.

For starters, Bevy Resvg is a very young and immature project. If
you are looking for something more mature and battle-tested, you should probably
use `bevy_svg`.

Another feature missing from Bevy Resvg is 3D rendering, which has first-class
support in `bevy_svg`. Unfortunately, you can't easily render to 3D objects with
Bevy Resvg, yet. Don't worry, it's on the [Todo-list](#todos)! In the meantime,
check out `bevy_svg` if you need to render to a 3D object.

Furthermore, if your game is dependent on zooming into the SVGs,
`bevy_svg` might fit your needs better (particularly until [custom render size
target support](#custom-render-size) is added). Bevy Resvg only performs
rasterisation once, mimicking the behaviour of e.g. the
[Godot Engine](https://forum.godotengine.org/t/how-to-make-svg-not-pixelated/92365).
This has the unfortunate side-effect of causing blurry images when zooming in.
`bevy_svg`, however, tesselates the SVGs into a crisp Mesh2d, which results in
sharper rendering when zoomed in.

However, if you are in need of rendering semi-transparent SVGs, then Bevy Resvg
is your only option (to my knowledge). Perhaps you even want to change the
runtime colour of your loaded SVGs. In that case, Bevy Resvg makes it super
simple! Just modify the outputted
[Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)'s `color`
field and you've got yourself a tinted SVG! That's not possible (again, to my
knowledge) in `bevy_svg`. What about gradients? No problem! Resvg handles
gradients naturally too.

I'm not sure why (and, like many other things in this project, I haven't
bothered to test why), but positioning and especially sizing SVGs with `bevy_svg`
as children of sprites felt clunky and required hard-coded values back when I
used `bevy_svg`. I have yet to encounter that issue with Bevy Resvg.

Also, although I have no data to back this up, I would assume that Bevy Resvg
might be a tiny bit faster than `bevy_svg`. Meshes *feel* more expensive than
simple textures to me, however I am no expert in this area.

During testing, I found that `bevy_svg` couldn't load complex Inkscape-generated
SVGs, while Bevy Resvg handled them flawlessly just like any other SVG file.
Although you should not ship Inkscape-generated SVGs in your released game, it
can be beneficial to not have to export your SVGs as standard SVGs from Inkscape
each time you want to test a new design.

### When to use bevy Resvg over Bevy Vello

To be written…

<!-- TODO: Write this section -->

## Usage

See the [examples](./examples/) directory for examples of how to use the Bevy
Resvg.

Currently, the only examples are:

- [`simple.rs`](./examples/simple.rs): shows the most basic usage of Bevy Resvg
- [`zoom.rs`](./examples/zoom.rs): shows what happens when you zoom too far into
  an SVG.

More examples are planned!

If you're too lazy to click the link, here's the contents of `simple.rs`:

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
    let svg: Handle<SvgFile> = asset_server.load("transparent.svg");
    commands.spawn(Camera2d);
    commands.spawn(Svg(svg));
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
3. Replace all occurences of `Svg` with `SvgFile` (make sure you're doing
   whole-word replacing. You don't want an `SvgFilePlugin`!).
4. Replace all occurences of `Svg2d` with `Svg`.

## Todos

- [ ] 3D-Rendering support
- [ ] JIT support! (because it's cool, I doubt that it is performant lol)
- [ ] Add more examples
- [ ] Add tests (there are currently none…)
- [ ] Custom rendering size targets (not dependent on `viewBox` value) <a id="custom-render-size"></a>
- [ ] Expand [comparison table](#comparison-between-bevy-resvg-bevy_svg-and-bevy-vello)
  - [ ] Particularly, add performance comparisons
- [x] Handle more
  [`AssetEvent`](https://docs.rs/bevy/latest/bevy/asset/enum.AssetEvent.html)s
  - [x] `Added`
  - [x] `Modified`
  - [x] `Removed`
  - [x] `Unused`
  - [x] `LoadedWithDependencies`
- [ ] [`usvg::Options`](https://docs.rs/usvg/latest/usvg/struct.Options.html)
  support
  - [ ] CSS support

## Minimum supported Rust version

Bevy Resvg's MSRV is 1.89 due to Bevy's MSRV.

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## Licence

[MIT](./LICENCE-MIT) OR [Apache-2.0](./LICENCE-APACHE), at your option.

[^1]: Calculated using [scc](https://github.com/boyter/scc), only counting Rust code.
[^2]: Based on commit
[`b3a3748` in `Weasy666/bevy_svg`](https://github.com/Weasy666/bevy_svg/commit/b3a3748b09ed1ea65eff634ed10142043b1f856e)
[^3]: Based on tagged release
[`v0.13.0` in `linebender/bevy_vello`](https://github.com/linebender/bevy_vello/tree/v0.13.0)
because later versions of Bevy Vello refuse to compile on my machine.
