# Changelog

## [2.5.0] - 2026-07-28

### Features

1. Add bsn support

### Other

1. Add bsn support

Add BSN (`bsn!`) template support for `Svg` and `UiSvg`

Derive `FromTemplate` and implement `AsAssetId` for `Svg` and `UiSvg` so
that asset paths can be written directly in `bsn!` scenes (e.g.
`UiSvg("icon.svg")`) without threading an `AssetServer` through scene
functions.

Thank you to @Mimikkk (Daniel Zdancewicz) for creating this PR!

### Documentation

1. Create a separate `simple_scene` example
2. Add U+FE0F to spider web
3. Update SLoC and complexity count

### Styling

1. *(fmt)* Run `cargo fmt`

### Miscellaneous Tasks

1. Remove dead reference to `sumi.toml`
2. Update bsn examples
3. *(release)* Show current version number

## [2.4.0] - 2026-06-25

### Features

1. Update Bevy to 0.19

Thank you to @atavistock (Aaron Tavistock) for creating this PR!
<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>Co-authored-by</td>
<td>Theodore Bjernhed <fosseder@danwin1210.de></td>
</tr>
</table>

### Miscellaneous Tasks

1. Prepare for 2.4 release

* feat: update my email address

* feat: update examples

* style(fmt): run `cargo fmt`

* docs(README): update Bevy references

* docs(README): fix grammar

* chore: remove `sumi.toml`

I put my `sumi.toml` in my own $XDG_CONFIG_DIR nowadays to avoid
cluttering my repos.

* docs(CONTRIBUTING): fix spelling mistake

2. Fix release script
3. *(release)* V2.4.0

## [2.3.3] - 2026-03-09

### Bugfixes

1. *(raster)* Actually handle post-load components

I forgot to test the previous attempt at handling post-load component
insertion. Due to the `read_events!` macro returning if the `svg_events`
`MessageReader` doesn't match any of the expected `AssetEvent`s, the
added check never actually did anything. Now it does.
<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>See</td>
<td>5048fc6c20d690e33ea35511df1d7febc4a77386</td>
</tr>
</table>

### Documentation

1. Update SLoC and complexity count

### Miscellaneous Tasks

1. *(release)* Use `git status --porcelain`

`git diff-index` sometimes returns 0 even the working tree is clean due
to a stale index stat cache. Use `git status --porcelain=1` instead,
which also has the added benefit of flagging untracked files.

2. *(release)* V2.3.3

## [2.3.2] - 2026-03-09

### Bugfixes

1. *(raster)* Handle post-load component insertion

Handle a timing gap where entities can miss `Sprite`/`ImageNode`
insertion if `Svg` (or `UiSvg`) is added after
`AssetEvent::LoadedWithDependencies` has already fired.

Update `handle_svg_loaded` and `handle_ui_svg_loaded` to query
`Ref<Svg>`/`Ref<UiSvg>` instead of `&Svg`/`&UiSvg` and insert render
components when either:

- The `Asset` ID appears in `LoadedWithDependencies`, or
- The `Svg` (or `UiSvg`) component was just added (`is_added()`).

### Documentation

1. Update SLoC and complexity count

### Styling

1. *(fmt)* Run cargo fmt

### Miscellaneous Tasks

1. *(release)* V2.3.2

## [2.3.1] - 2026-03-09

### Documentation

1. Update SLoC and complexity count

### Miscellaneous Tasks

1. *(release)* Add script to update metrics

Add a script written by ChatGPT that updates the `README.md` comparison
table's Source Lines of Code and Complexity values for Bevy Resvg. I
keep forgetting to update it myself, so a script will be helpful.
ChatGPT wrote it since I don't know any `awk` syntax.

2. *(release)* V2.3.1

## [2.3.0] - 2026-03-08

### Features

1. Add `usvg::Options` support

### Other

1. `serde-remote` into `main`

An alternative solution to the one used in `resvg-option`. Instead of
patching `usvg`, we can use `#[serde(remote = "…")]` together with
`#[serde(with = "…")]` to derive De/Serialize for types in `usvg`, as
described in the serde documentation[^1].

Properly implementing this involves a lot of boilerplate, so I wrote two
macros: `enum_def` and `options_def`. All enums use `enum_def`, and
`OptionsDef` uses the appropriately named `options_def` macro. `SizeDef`
was small enough that I found it easier to just write the impls by hand
instead of making a macro.

I attempted to stay as far away from any custom syntax as possible, but
in the end I added an `-> Ident` to the signatures to represent the
remote types. This simplified the macro code, and should *hopefully* not
be *too* confusing.

This has the disadvantage of not allowing us to include every option in
our own `OptionsDef`. However, I still consider this solution
good-enough for now, especially considering that I am not even allowed
to publish the alternative solution used in `resvg-option`.

However, if one still requires configuring one of the options that
`OptionsDef` doesn't expose (namely `image_href_resolver`,
`font_resolver`, and `fontdb`), then one will still be able to use the
`resvg-option` branch. It is important that one follows the instructions
as described in the `USAGE.md` file in the `resvg-option` branch's root.
<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>[^1]</td>
<td><https://serde.rs/remote-derive.html></td>
</tr>
</table>

### Refactor

1. Simplify macros

### Documentation

1. *(spelling)* Fix spelling and grammar mistakes
2. *(README)* Add `resvg-option` footnote
3. *(README)* `target_render_size` is not a method
4. Add documentation and examples for `Options`
5. Fix markdown for rustdoc
6. Fix grammar

### Miscellaneous Tasks

1. *(release)* V2.3.0

## [2.2.0] - 2026-03-07

### Features

1. Add target render size

### Documentation

1. *(release)* Don't skip any proper commits
2. *(release)* Correct whitespace in changelog
3. Update SLoC and complexity count

### Miscellaneous Tasks

1. *(sumi)* Add `revert` as commit type
2. Temporary commit
3. *(deps)* Don't require patch version
4. *(release)* V2.2.0

### Revert

1. Chore: don't put chore(release) in `CHANGELOG.md`

<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>Refs</td>
<td>a2c0f9d5fbd28edd84d6f2fe2e2718f05bd1bb6a</td>
</tr>
</table>

## [2.1.0] - 2026-03-07

### Features

1. Add colour tinting support

Parts taken from [生于斯](https://github.com/shengyusi-SYS)'s
[fork of this repository](https://github.com/shengyusi-SYS/bevy_svg_ui)

### Documentation

1. *(examples)* Add color examples
2. *(README)* Update `README.md` for new examples
3. *(README,style)* Remove `.rs` from example list
4. *(release)* Use html table instead of md table

Markdown tables break when the value is multi-line. `HTML` tables do
not.

5. *(release)* Simplify whitespace
6. *(release)* Update `CHANGELOG.md` to new format

### Miscellaneous Tasks

1. *(release)* V2.1.0

## [2.0.0] - 2026-03-07

### Features

1. Explicitly state which files are supported
2. Add SVG type for UI rendering

### Bugfixes

1. Properly handle all `AssetEvent`s

- Replace single monolithic event handler system with dedicated handlers
  for each `AssetEvent` variant.
- Add support for the `Removed` and `Unused` events.
- Properly handle the `Modified` event, which didn't actually update the
  `Sprite before`

<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>NOTE</td>
<td>`Added` isn't actually handled, but it isn't supposed to either.
We react to `LoadedWithDependencies` instead.</td>
</tr>
<tr>
<td>Fix</td>
<td>#1</td>
</tr>
</table>

2. *(README)* Correct factual error about bevy_svg

### Other

1. `all-asset-events` into `main`

### Documentation

1. Check off more `AssetEvent`s on todo-list
2. Update status for `bevy_svg`
3. Update our support for hot-reloading
4. Add comparison with Bevy Vello

<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>Link</td>
<td>https://github.com/linebender/bevy_vello</td>
</tr>
</table>

5. Add JIT to Todo list
6. Include note about Inkscape SVGs
7. Add UI example
8. *(README)* Update `README.md` to mention UI
9. *(spelling)* Fix spelling mistake in doccomment
10. *(release)* Don't use h4
11. *(release)* Update `CHANGELOG.md` format

### Performance

1. Use `HashSet` instead of `Vec` for events

`HashSet`s are supposedly faster than `Vec`s for `contain` calls,
although I haven't actually tested it.

2. Check if asset id list is empty before loop

### Styling

1. *(sumi)* Sort allowed types array
2. *(README)* Fix hard-wrapping in README

### Miscellaneous Tasks

1. *(sumi)* Add `merge` as commit type
2. *(deps)* Bump dependencies
3. *(deps)* Don't require patch version
4. Add `debug` as bevy dev feature
5. Enable default features for bevy dev builds
6. Add helper event function
7. *(release)* Ignore non-conventional commits
8. *(release)* Create release script
9. *(release)* V2.0.0

## [1.0.1] - 2026-01-24

### Bugfixes

1. Make `zoom.rs` example compile

### Other

1. *(release)* Release v1.0.1

### Documentation

1. Add badges
2. Clarify how SLoC is counted
3. Add documentation for each SvgError variant
4. Update SLoC count
5. *(release)* Create changelog for v1.0.0
6. *(release)* Update `CHANGELOG.md` for 1.0.1

### Styling

1. Make clippy like my `README.md` file
2. Remove superfluous `default` call in zoom

### Miscellaneous Tasks

1. Restrict visibility of internal method
2. Add git-cliff
3. Exclude `cliff.toml` file
4. Configure git-cliff
5. *(release)* Add `release.toml` file
6. Exclude `release.toml` file
7. *(release)* Add more release configurations
8. Don't put chore(release) in `CHANGELOG.md`

## [1.0.0] - 2026-01-24

### Features

1. Create a boilerplate SVG-loading plugin
2. Add initial working version
3. Relax dependencies
4. Warn when unimplemented events are emitted
5. Make internal types private
6. Add example for what happens when you zoom
7. Add prelude

### Refactor

1. [*BREAKING*] Rename types to be less technical

### Documentation

1. Document code
2. Rewrite documentation to remove dead links
3. Add `CONTRIBUTING.md`
4. Add content to `README.md`
5. Add migration guide from bevy_svg
6. Remove unsupported angle brackets for links

### Styling

1. *(Cargo.toml)* Sort `package` field
2. Sort `description` field
3. Sort `exclude` field

### Miscellaneous Tasks

1. Initial commit
2. Release version 1.0.0
3. Add categories, keywords and a description
4. Exclude `sumi.toml` file from <crates.io>
