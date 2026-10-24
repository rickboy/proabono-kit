#!/usr/bin/env bash
# Build everything GitHub Pages serves into dist/:
#   dist/r/         component registry (proabono build)
#   dist/index.html landing page rendered from dist/r/index.json
#   dist/gallery/   static snapshot of the gallery site
#
# Needs: cargo, python3, curl, and the tailwindcss standalone CLI on PATH.
set -euo pipefail
cd "$(dirname "$0")/.."
ROOT=$PWD
PORT=${GALLERY_PORT:-3999}

cargo build --release --locked --manifest-path cli/Cargo.toml
cli/target/release/proabono build
python3 pages/render_landing.py dist/r/index.json > dist/index.html

cd site
mkdir -p "$ROOT/dist/gallery/static/proa/proabono"
# Fresh CSS straight into the snapshot; the committed public/styles.css is left alone.
tailwindcss -i styles/input.css -o "$ROOT/dist/gallery/static/styles.css" --minify
cargo build --release --locked
PORT=$PORT PROA_HOST=127.0.0.1 ./target/release/site > "$ROOT/dist/.gallery.log" 2>&1 &
SERVER=$!
trap 'kill $SERVER 2>/dev/null || true' EXIT
for _ in $(seq 1 60); do
  curl -sf "http://127.0.0.1:$PORT/healthz" > /dev/null && break
  sleep 1
done
cd "$ROOT"
python3 pages/export_gallery.py "http://127.0.0.1:$PORT" dist/gallery
cp site/public/favicon.svg dist/gallery/static/
cp registry/core/assets/* dist/gallery/static/proa/proabono/
rm -f dist/.gallery.log
touch dist/.nojekyll
echo "pages built in dist/"
