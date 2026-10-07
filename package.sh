#!/usr/bin/env bash
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
version="${VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' "$here/Cargo.toml" | head -n1)}"
id="blank-app"

cargo build --release --manifest-path "$here/Cargo.toml"

stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/bin/linux"
cp "$here/target/release/$id" "$stage/bin/linux/$id"
command -v strip >/dev/null && strip "$stage/bin/linux/$id"
cp "$here/assets/icon.png" "$here/assets/background.png" "$stage/"

run_windows=""
if [[ -n "${WINDOWS_EXE:-}" ]]; then
    mkdir -p "$stage/bin/windows"
    cp "$WINDOWS_EXE" "$stage/bin/windows/$id.exe"
    run_windows=",
    \"windows\": { \"command\": \"bin/windows/$id.exe\" }"
fi

git_out() { git -C "$here" "$@" 2>/dev/null || true; }
commit="$(git_out rev-parse --verify HEAD)"
branch="$(git_out rev-parse --abbrev-ref HEAD)"
repo="$(git_out remote get-url origin | sed -e 's/\.git$//' -e 's#^git@\([^:]*\):#https://\1/#')"
tag="$(git_out describe --tags --exact-match)"
dirty=false
[[ -n "$(git_out status --porcelain -- "$here")" ]] && dirty=true

json_or_null() { [[ -n "$1" ]] && printf '"%s"' "$1" || printf 'null'; }

cat > "$stage/yasaffConfig.json" <<EOF
{
  "yasaff": "app",
  "formatVersion": 1,
  "id": "$id",
  "name": "Blank App",
  "version": "$version",
  "kind": "app",
  "author": "Krystian Grzelak",
  "summary": "An empty app with only a Quit button",
  "description": "A starting point for new YASAFF apps: a window held inside YASAFF that does nothing yet.\n\nPress B (or Esc, or click Quit) to go back to YASAFF.",
  "icon": "icon.png",
  "background": "background.png",
  "run": {
    "linux": { "command": "bin/linux/$id" }$run_windows
  },
  "install": {
    "executables": ["bin/linux/$id"]
  },
  "git": {
    "repository": $(json_or_null "$repo"),
    "branch": $(json_or_null "$branch"),
    "commit": $(json_or_null "$commit"),
    "tag": $(json_or_null "$tag"),
    "dirty": $dirty
  }
}
EOF

mkdir -p "$here/dist"
out="$here/dist/$id-$version.zip"
rm -f "$out"
(cd "$stage" && zip -qr "$out" .)
echo "$out"
