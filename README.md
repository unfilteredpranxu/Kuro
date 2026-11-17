# KURO // STUDY OS

A Rust-first, local-first study workspace. Dioxus **0.7.10**, Rust **1.100.0**. Website foundation; **not a production-complete Study OS**.

## Run

On this cloud machine:

```sh
cd /workspace/Kuro
bash scripts/bootstrap-cloud.sh
. scripts/env.sh
dx serve --platform web --package kuro-web --addr 0.0.0.0 --port 4173 --open false --interactive false
```

On an ordinary development machine, install the pinned Rust toolchain, Node 24 and Dioxus CLI 0.7.10; the same Cargo workspace works without cloud environment variables. The cloud bootstrap is specifically Linux x86_64. Use the existing isolated checkout; no worktree is needed.

For a standalone build:

```sh
. scripts/env.sh
dx build --platform web --package kuro-web --release --locked
python3 -m http.server 4173 --bind 0.0.0.0 --directory target/dx/kuro-web/release/web/public
```

Serve the output over HTTP(S) at the origin root; opening index.html via file:// will not run WebAssembly. IndexedDB storage belongs to the exact browser origin and profile. A different port or deployment does not share data; use backup/restore to move it.

## Implemented

- Original provisional editorial/pixel-art shell, responsive navigation and command palette (Ctrl/Cmd+K).
- Five persistent themes, real empty states and honest unavailable-module screens.
- Tasks: create, edit title, reschedule, complete/reopen, dated planner agenda and filters.
- Typed notes with chapter labels, autosave and refresh recovery drafts.
- Question attempt log, independent-accuracy calculation, mistakes linked to revision cards.
- Recall cards with due dates and 1/3/7/14/30-day intervals adjusted by recall outcome.
- Transactional IndexedDB saves, stale-tab protection, visible failed writes and protected malformed state.
- Validated, SHA-256-checked backup export and restore, including restore into a clean browser profile.

## Deliberately incomplete

Official syllabus ingestion and topic tracking are blocked on access to the official source. The syllabus screen links the requested **2026 baseline**, without calling it the confirmed 2027 syllabus. PDF import/rendering/annotation, handwriting, complete calendar/recurrence, study timer, exam scoring, wallpaper import, PWA asset caching, streaming providers and all native applications remain unimplemented. Music is a disconnected visual study only; no real playback or authentication is claimed. Chapter labels are user labels, not audited official topic IDs.

See [engineering report](docs/ENGINEERING-REPORT.md), [source gate](docs/SYLLABUS.md), [architecture and storage limitations](docs/ARCHITECTURE.md), and [provisional design grammar](docs/DESIGN.md).

## Verify

```sh
bash scripts/check.sh
npm ci --ignore-scripts
# System Chromium is used when available; otherwise:
npx playwright install chromium
npm run test:e2e
```

The browser runner starts a static server for the release build on dedicated port 4174. It refuses to reuse another server, ensuring tests exercise the release artifact. Visual baselines are reviewed captures of this provisional design, not the missing user references. Never update them solely to make a diff pass.

No native targets are enabled in CI until native implementations exist. GitHub Actions configuration is present but has not been executed on GitHub in this session.

## Downloadable build artifact

Source code, lockfiles, tests, documentation and screenshots belong in Git. Generated `target/`, dependencies and `dist/` are excluded. After the release build, run `python3 scripts/package_web.py` to generate `dist/kuro-web-0.1.0.zip` and its checksum file. The existing ZIP is retained separately as a cloud artifact; it does not replace the source repository.
