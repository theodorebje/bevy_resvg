# Bevy Resvg

A simple library for rendering SVGs in Bevy using the amazing
[Resvg](https://github.com/linebender/resvg) library.

## Why not [bevy_svg](https://github.com/Weasy666/bevy_svg)?

I originally tried forking bevy_svg to add colour tinting support to the SVGs,
as proposed by [bevy_svg/#54](https://github.com/Weasy666/bevy_svg/issues/54).
However, I quickly noticed that the bevy_svg had so much technical debt
(no offense to [Weasy666](https://github.com/Weasy666/), your library has
clearly worked for many, many people) due to its 5 years of existence that it
was just easier to start a new library from scratch.

Funnily enough, I'm actually publishing this crate onto <crates.io> on
bevy_svg's 5-year anniversary
([January 24th](https://github.com/Weasy666/bevy_svg/commit/b4d9041))!

**Bevy Resvg** takes a completely different approach to rendering SVGs compared
to bevy_svg. Instead of tesselating the SVG into a mesh, it first renders the
SVG as a raster image using the [Resvg](https://github.com/linebender/resvg)
library. This has both advantages and (unfortunately) drawbacks over bevy_svg's
approach:

### Comparison between Bevy Resvg and bevy_svg

|          Feature          |    |           Bevy Resvg           |    |      bevy_svg       |
| ------------------------- | -- | ------------------------------ | -- | ------------------- |
| SLoC                      | 🔽 |                        199[^1] | 🔼 |       1912[^1][^2]  |
| Code Complexity           | 😀 |                         11[^1] | 😵 |        145[^1][^2]  |
| Changing runtime color    | ✅ | Supported                      | ❌ | Unsupported         |
| Gradients                 | ✅ | Supported                      | ❌ | Unsupported         |
| Semi-transparency         | ✅ | Supported                      | ❌ | Unsupported         |
| Positioning and sizing    | ✅ | Native (`Sprite`-based)        | ❌ | Janky and imprecise |
| Static SVG Spec Support   | ✅ | Fully Supported                | ⚠️ | Partial Support     |
| Rendered quality (normal) | ✅ | Crisp                          | ✅ | Crisp               |
| Rendered quality (zoomed) | ❌ | Blurry and Pixelated           | ✅ | Crisp               |
| Hot-reloading of SVGs     | ❌ | Unsupported                    | ❌ | Unsupported         |
| Animated SVGs             | ❌ | Unsupported                    | ❌ | Unsupported         |
| Approach                  | 🖼️ | Rasterisation                  | 🔺 | Tesselation         |
| Output                    | 🏃‍➡️ | Sprite                         | 🕸  | Mesh2d              |
| Licence                   | 🟰 | MIT OR Apache-2.0              | 🟰 | MIT OR Apache-2.0   |

…to be expanded…

### When to use Bevy Resvg over bevy_svg

Although I am very proud of this small little crate (it's my first ever library
to be published on <crates.io>!), I do realise that there are situations in
which bevy_svg simply makes more sense.

For starters, Bevy Resvg is a very young and immature project. If
you are looking for something more mature and battle-tested, you should probably
use bevy_svg.

Furthermore, if your game is dependent on zooming into the SVGs,
bevy_svg might fit your needs better (particularly until [custom render size
target support](#custom-render-size) is added). Bevy Resvg only performs
rasterisation once, mimicking the behaviour of e.g. the
[Godot Engine](https://forum.godotengine.org/t/how-to-make-svg-not-pixelated/92365).
This has the unfortunate side-effect of causing blurry images when zooming in.
bevy_svg, however, tesselates the SVGs into a crisp Mesh2d, which results in
sharper rendering when zoomed in.

However, if you are in need of rendering semi-transparent SVGs, then Bevy Resvg
is your only option (to my knowledge). Perhaps you even want to change the
runtime colour of your loaded SVGs. In that case, Bevy Resvg makes it super
simple! Just modify the outputted
[Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)'s `color`
field and you've got yourself a tinted SVG! That's not possible (again, to my
knowledge) in bevy_svg. What about gradients? No problem! Resvg handles
gradients naturally too.

I'm not sure why (and, like many other things in this project, I haven't
bothered to test why), but positioning and especially sizing SVGs with bevy_svg
as children of sprites felt clunky and required hard-coded values back when I
used bevy_svg. I have yet to encounter that issue with Bevy Resvg.

Also, although I have no data to back this up, I would assume that Bevy Resvg
might be a tiny bit faster than bevy_svg. Meshes *feel* more expensive than
simple textures to me, however I am no expert in this area.

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

## Todos

- [ ] Add more examples
- [ ] Add tests (there are currently none…)
- [ ] Custom rendering size targets (not dependent on `viewBox` value) <a id="custom-render-size"></a>
- [ ] Expand [comparison table](#comparison-between-bevy-resvg-and-bevy_svg)
  - [ ] Particularly, add performance comparisons
- [ ] Handle more
[`AssetEvent`](https://docs.rs/bevy/latest/bevy/asset/enum.AssetEvent.html)s
  - [x] `Added`
  - [x] `Modified`
  - [ ] `Removed`
  - [ ] `Unused`
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

[^1]: Calculated using [scc](https://github.com/boyter/scc).
[^2]: Based on the not-yet-merged
[0.18 PR](https://github.com/Weasy666/bevy_svg/pull/59) by
[cilki](https://github.com/cilki).
