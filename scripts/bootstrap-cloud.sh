#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p /workspace/tooling/{cargo,bin,cache,config,data}
. scripts/env.sh
if ! command -v rustup >/dev/null; then
  curl -fsSL https://sh.rustup.rs -o /workspace/tooling/rustup-init.sh
  sh /workspace/tooling/rustup-init.sh -y --profile minimal --default-toolchain 1.100.0 --no-modify-path
fi
rustup toolchain install 1.100.0 --profile minimal --component clippy --component rustfmt --target wasm32-unknown-unknown
if ! command -v dx >/dev/null || ! dx --version | head -1 | grep -q '0.7.10'; then
  curl -fsSL https://github.com/DioxusLabs/dioxus/releases/download/v0.7.10/dx-x86_64-unknown-linux-gnu.tar.gz -o /workspace/tooling/dx.tar.gz
  tar -xzf /workspace/tooling/dx.tar.gz -C /workspace/tooling/bin dx
fi
# dx's embedded HTTPS client does not trust this cloud proxy CA. Use the
# system-trusted npm/curl clients, preserving TLS and package integrity checks.
if [ ! -x "$XDG_DATA_HOME/.dx/tools/esbuild-0.27.3/esbuild" ]; then
  npm install --prefix /workspace/tooling/esbuild-install --ignore-scripts --cache /workspace/tooling/npm-cache @esbuild/linux-x64@0.27.3
  mkdir -p "$XDG_DATA_HOME/.dx/tools/esbuild-0.27.3"
  cp /workspace/tooling/esbuild-install/node_modules/@esbuild/linux-x64/bin/esbuild "$XDG_DATA_HOME/.dx/tools/esbuild-0.27.3/esbuild"
fi
if [ ! -x "$XDG_DATA_HOME/.dx/tools/wasm-bindgen-0.2.130/wasm-bindgen" ]; then
  cargo install wasm-bindgen-cli --version 0.2.130 --locked --root /workspace/tooling/bindgen-install
  mkdir -p "$XDG_DATA_HOME/.dx/tools/wasm-bindgen-0.2.130"
  cp /workspace/tooling/bindgen-install/bin/wasm-bindgen "$XDG_DATA_HOME/.dx/tools/wasm-bindgen-0.2.130/wasm-bindgen"
fi
if [ ! -x "$XDG_DATA_HOME/.dx/tools/binaryen-129/bin/wasm-opt" ]; then
  curl -fsSL https://github.com/WebAssembly/binaryen/releases/download/version_127/binaryen-version_127-x86_64-linux.tar.gz -o /workspace/tooling/binaryen.tar.gz
  tar -xzf /workspace/tooling/binaryen.tar.gz -C /workspace/tooling binaryen-version_127/bin/wasm-opt
  # dx 0.7.10 itself downloads Binaryen 127 into a cache named binaryen-129.
  mkdir -p "$XDG_DATA_HOME/.dx/tools/binaryen-129/bin"
  cp /workspace/tooling/binaryen-version_127/bin/wasm-opt "$XDG_DATA_HOME/.dx/tools/binaryen-129/bin/wasm-opt"
fi
npm ci --ignore-scripts --cache /workspace/tooling/npm-cache
rustc --version
dx --version
