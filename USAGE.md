# Patching `usvg`

In order to use this branch of Bevy Resvg, you must first patch `usvg` to
support Serde. Simply put this in your `Cargo.toml` file:

```toml
[dependencies.bevy_resvg]
git = "https://github.com/theodorebje/bevy_resvg.git"
branch = "resvg-option"

[patch.crates-io]
usvg = { git = 'https://github.com/theodorebje/usvg-serde.git' }
```

Make sure that you delete your regular `bevy_resvg` dependency from the
`[dependencies]` section and replace it with the Git version.
