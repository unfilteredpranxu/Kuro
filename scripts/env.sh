#!/usr/bin/env bash
# Ordinary local Rust installations also work; isolate cloud-local caches.
if [ -d /workspace/tooling/cargo ]; then
  export CARGO_HOME=/workspace/tooling/cargo
  export RUSTUP_HOME=/workspace/tooling/rustup
  export PATH=/workspace/tooling/cargo/bin:/workspace/tooling/bin:$PATH
  export XDG_CACHE_HOME=/workspace/tooling/cache
  export XDG_CONFIG_HOME=/workspace/tooling/config
  export XDG_DATA_HOME=/workspace/tooling/data
fi
