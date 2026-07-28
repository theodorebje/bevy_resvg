#!/bin/bash

set -ex # Exit if there's an error

NO_PUBLISH=false

for arg in "$@"; do
    case $arg in
        --no-publish)
            NO_PUBLISH=true
            ;;
    esac
done

# Dry-run helper
run() {
    if $NO_PUBLISH; then
        echo "[no-publish] $*"
    else
        "$@"
    fi
}

if $NO_PUBLISH; then
  echo "Running in no-publish mode. Not publishing changes."
fi

# Ensure working tree is clean
CHANGED_FILES="$(git status --porcelain=1)"
if [ -n "${CHANGED_FILES}" ]; then
    echo "Working tree is not clean. Commit or stash your changes first."
    echo "Changes since last commit:"
    echo "${CHANGED_FILES}"
    exit 1
fi

echo "Checking that project compiles"
cargo check --all-targets

OLD_VERSION_NUMBER="$(cargo pkgid | cut -d '#' -f2)"
echo "Current version number: $OLD_VERSION_NUMBER"
read -p "New version number: " VERSION
TAG="v${VERSION}"

echo "Preparing release ${TAG}"

echo "Updating comparison metrics"
bash update_readme_metrics.bash

git add README.md
git commit -m "docs: update SLoC and complexity count" || true

echo "Generating CHANGELOG.md"
git-cliff --tag "${TAG}" -o CHANGELOG.md

echo "Updating version number in Cargo.toml"
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

git add CHANGELOG.md Cargo.toml
git commit -m "chore(release): ${TAG}"

git tag -a "${TAG}" -m "Release ${TAG}"

run git push
run git push origin "${TAG}"

if $NO_PUBLISH; then
    echo "Dry-run cargo publish"
    cargo publish --dry-run
    echo "The following files will be included:"
    cargo package --list
    echo "Dry-run complete. Please note that a git tag and commit was created."
else
    echo "Publishing to crates.io"
    cargo publish
    echo "Release ${TAG} created and pushed."
fi
