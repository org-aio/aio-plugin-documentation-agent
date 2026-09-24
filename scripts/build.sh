#!/bin/sh
set -eu
cd "$(dirname "$0")/.."

node scripts/generate-aio-pages.mjs

cd frontend
npm ci --ignore-scripts
npm run build
cd ..

cargo zigbuild --locked --release \
  --target x86_64-unknown-linux-gnu.2.17 \
  --manifest-path backend-topcoat/Cargo.toml

rm -rf dist/frontend
mkdir -p dist/frontend
cp -R frontend/dist/. dist/frontend/
cp backend-topcoat/target/x86_64-unknown-linux-gnu/release/boxun-topcoat-server \
  dist/boxun-topcoat-server
chmod 0755 dist/boxun-topcoat-server
