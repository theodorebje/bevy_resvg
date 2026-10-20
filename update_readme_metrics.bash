#!/bin/bash
# Written by ChatGPT because I have no idea on how to use awk lol

set -euo pipefail

# Extract Code and Complexity from scc, preferring "Total" over "Rust"
metrics="$(scc -i rs --format csv | awk -F',' '
    function trim(s) { gsub(/^[[:space:]"]+|[[:space:]"]+$/, "", s); return s }

    NR == 1 {
        for (i = 1; i <= NF; i++) {
            h = trim($i)
            if (h == "Code")      code_col = i
            if (h ~ /Complexity/) cplx_col = i
        }
        next
    }
    {
        lang = trim($1)
        if (lang != "Rust" && lang != "Total") next

        code = trim($code_col)
        cplx = trim($cplx_col)
        if (code == "" || cplx == "") next

        # "Total" always wins; "Rust" is only used when nothing better was seen
        if (lang == "Total" || !found) {
            best_code = code
            best_cplx = cplx
            found = (lang == "Total") ? 2 : 1
        }
    }
    END { if (!found) exit 1; print best_code "," best_cplx }
')"

loc="${metrics%%,*}"
complexity="${metrics##*,}"

[[ -n "$loc" && -n "$complexity" ]] || {
    echo "error: failed to parse Code/Complexity values from scc CSV" >&2
    exit 1
}

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

# Replace the Bevy Resvg column value in the matching table rows.
# Note: the [^1] footnote marker now lives on the label instead of the value,
# so match the row by its label and update the value column (4th |-delimited field).
awk -F'|' -v OFS='|' -v loc="$loc" -v complexity="$complexity" '
    $2 ~ /Source Lines of Code/ {
        gsub(/[0-9][0-9,]*/, loc, $4)
        loc_updated = 1
    }
    $2 ~ /Code Complexity/ {
        gsub(/[0-9][0-9,]*/, complexity, $4)
        complexity_updated = 1
    }
    { print }
    END { if (!loc_updated || !complexity_updated) exit 2 }
' README.md > "$tmp"

mv "$tmp" README.md

echo "Updated README.md with Source Lines of Code=$loc and Code Complexity=$complexity"