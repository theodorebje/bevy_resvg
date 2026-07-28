#!/usr/bin/env bash
set -euo pipefail

REPO="theodorebje/bevy_resvg"
BRANCH="main"

declare -A DESC
DESC[simple]="shows the most basic usage of Bevy Resvg"
DESC[simple_scene]="shows basic bsn! and World::spawn_scene integration in Bevy Resvg"
DESC[color]="shows how to apply a static colour tint to an Svg"
DESC[color_change]="shows how to update the colour tint of an Svg when an event (spacebar pressed) occurs"
DESC[color_hue_tween]="showcase of an Svg that continuously tweens through all hues"
DESC[color_ui]="shows how to apply a static colour tint to a UiSvg"
DESC[color_ui_change]="shows how to update the colour tint of a UiSvg when an event (spacebar pressed) occurs"
DESC[color_ui_hue_tween]="showcase of a UiSvg that continuously tweens through all hues"
DESC[crisp_shapes]="shows how to customise usvg::Options to render SVGs with crisp edges and with anti-aliasing turned off"
DESC[custom_aspect_ratio]="shows how to render an SVG to a custom target size with a non-native aspect ratio"
DESC[custom_size]="shows how to render an SVG to a custom target size"
DESC[custom_style_sheet]="shows how to customise usvg::Options while loading an SVG by setting style_sheet"
DESC[ui]="shows how to render UiSvgs in UI nodes"
DESC[zoom]="shows what happens when you zoom too far into an SVG"

html_escape() {
  sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g; s/"/\&quot;/g; s/'"'"'/\&#39;/g'
}

basic_md_to_html() {
  sed \
    -e 's/^### \(.*\)/<h3>\1<\/h3>/' \
    -e 's/^## \(.*\)/<h2>\1<\/h2>/' \
    -e 's/\*\*\([^*]*\)\*\*/<strong>\1<\/strong>/g' \
    -e 's/`\([^`]*\)`/<code>\1<\/code>/g' \
    -e 's/^- \(.*\)/<li>\1<\/li>/'
}

generate_index() {
  local examples=("$@")
  cat > out/index.html <<- INDEX
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>bevy_resvg examples</title>
</head>
<body>
<h1>bevy_resvg — WASM examples</h1>
<ul>
INDEX
  for name in "${examples[@]}"; do
    local label="${name//_/ }"
    cat >> out/index.html <<- ITEM
<li><a href="${name}.html">${label}</a></li>
ITEM
  done
  cat >> out/index.html <<- INDEX
</ul>
</body>
</html>
INDEX
}

generate_example_page() {
  local name="$1"
  local readme_desc="$2"
  local src_file="examples/${name}.rs"

  local doc_lines=""
  local code_body=""
  if [[ -f "$src_file" ]]; then
    doc_lines=$(sed -n '/^\/\/\!/p' "$src_file" | sed 's/^\/\/! \?//')
    code_body=$(sed '/^\/\/\!/d' "$src_file")
  fi

  local doc_html=""
  if [[ -n "$doc_lines" ]]; then
    doc_html=$(echo "$doc_lines" | basic_md_to_html)
  fi

  local code_escaped
  code_escaped=$(echo "$code_body" | html_escape)

  local github_url="https://github.com/${REPO}/blob/${BRANCH}/examples/${name}.rs"

  cat > "out/${name}.html" <<- PAGE
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>${name} — bevy_resvg example</title>
</head>
<body>
<a href="./">← back</a>

<h1>${name}</h1>

<p>${readme_desc}</p>

${doc_html}

<a href="${github_url}">view source on GitHub</a>

<h2>Source</h2>
<pre><code>${code_escaped}</code></pre>

<h2>Live</h2>
<script type="module">
import init from "./wasm/${name}.js";
init();
</script>
</body>
</html>
PAGE
}

main() {
  local examples=()
  for ex in examples/*.rs; do
    [[ -f "$ex" ]] || continue
    examples+=("$(basename "$ex" .rs)")
  done

  echo "Building ${#examples[@]} examples for wasm32-unknown-unknown..."
  for name in "${examples[@]}"; do
    echo "  building $name"
    cargo build --target wasm32-unknown-unknown --release --example "$name"
  done

  mkdir -p out/wasm
  echo ""
  echo "Generating JS bindings..."
  for name in "${examples[@]}"; do
    echo "  wasm-bindgen $name"
    wasm-bindgen \
      --out-dir out/wasm \
      --target web \
      "target/wasm32-unknown-unknown/release/examples/${name}.wasm"
  done

  if command -v wasm-opt &> /dev/null; then
    echo ""
    echo "Running wasm-opt..."
    for wasm in out/wasm/*.wasm; do
      echo "  $(basename "$wasm")"
      wasm-opt -Oz -o "$wasm" "$wasm"
    done
  else
    echo "wasm-opt not found — skipping optimization"
  fi

  echo ""
  echo "Generating HTML pages..."
  generate_index "${examples[@]}"
  for name in "${examples[@]}"; do
    generate_example_page "$name" "${DESC[$name]:-}"
  done

  echo ""
  echo "Copying assets..."
  cp -r assets out/ 2>/dev/null || true

  echo ""
  echo "Done — output in out/"
}

main
