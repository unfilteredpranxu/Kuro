# KURO engineering checkpoint — 17 November 2026

This is a verified first web increment, not a production-complete Study OS. Work is saved in the repository. No deployment, native app, background development, GitHub CI run or published environment snapshot is claimed.

## Milestone status

| Milestone | Implemented and verified | Gate still open |
| --- | --- | --- |
| M0 — repository and architecture | Cargo workspace, typed shared Rust core, Dioxus 0.7.10 WebAssembly app, pinned tools and locks, repeatable cloud setup, browser tests, web CI configuration, built and launched release artifact | Only initial repository/music contracts exist. Full native platform adapters and native applications are future work. CI configuration has not run on GitHub. |
| M1 — design and shell | Original pixel observatory artwork, editorial shell, responsive module navigation, compact sidebar, keyboard command palette with focus handling, five saved themes, authentic empty states | Provisional visual foundation: five promised reference screenshots were not attached, and online references were inaccessible. Reference-based final design acceptance remains open. |
| M2 — official syllabus and persistence | Transactional browser persistence, eight typed topic-status variants and source acquisition script; source-linked blocked-state screen | **Blocked/incomplete:** official PDF is denied by network policy. No official topics have been ingested, no complete audit exists, no topic workspace/status UI is enabled, and a 2027 syllabus is not verified. |

While the official-source gate was blocked, useful independent slices were implemented: dated tasks and agenda, typed notes, question attempt records, mistake-to-revision links, recall scheduling, honest aggregate metrics and backup/restore. These do not complete M3–M9. In particular the planner is a dated agenda with a read-only default routine, not a complete drag-and-drop or recurring calendar.

## Exact working scope

- Task create, title edit, reschedule, complete and reopen; filters; the planner reads the same task records. Multi-day chapters can be represented by separate dated tasks, without automatically granting mastery.
- Plain-text notes with user chapter labels, 600ms autosave, explicit save, per-tab refresh recovery drafts and protected switching between notes. Notes render literal text, including HTML-like input.
- Attempt outcomes: independent, hinted, incorrect and skipped. Independent accuracy excludes skipped records from the denominator and never credits hinted answers as independently correct.
- A recorded mistake creates a linked recall card. Cards are immediately due; recall outcomes drive the 1/3/7/14/30-day checkpoints and review count. A created card does not count as a completed review.
- Theme and study records persist through reload. Writes validate, compare the stored revision, and commit transactionally before success is displayed. Stale tabs and failed writes cannot silently replace committed records.
- Portable backup envelope with schema, format, exact payload and SHA-256; malformed/unsupported/corrupt data is rejected. Restore requires an explicit replace-data acknowledgement. A clean browser profile successfully restored exported data. Oversized exports are refused instead of producing backups the importer cannot restore.
- Module navigation, Ctrl/Cmd+K search, focus rings, skip link, reduced-motion handling, responsive layout and licensed local fonts. No fabricated achievements or study hours.

## Relevant files changed

- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`: workspace and reproducible dependencies.
- `crates/kuro-core/src/lib.rs`: typed records, validation, accuracy, revision and backup algorithms/tests.
- `apps/web/src/main.rs`, `apps/web/src/store.rs`: actual Dioxus UI and transaction-backed application state.
- `apps/web/public/storage.js`: browser storage, file and recovery-draft adapters; study algorithms remain Rust.
- `apps/web/public/style.css`, `observatory.svg`, `fonts/`, `apps/web/index.html`: original design foundation, local licensed fonts and document metadata.
- `tests/e2e/`, `playwright.config.js`, `.github/workflows/web.yml`: workflow, failure, accessibility and screenshot checks.
- `scripts/bootstrap-cloud.sh`, `env.sh`, `check.sh`, `package_web.py`: tested setup/build/package tooling.
- `scripts/fetch_syllabus.py`, `docs/SYLLABUS.md`: official-source acquisition and explicit audit continuation gate. The real fetch failed; only Python syntax is independently validated.

## Build and test evidence

Final outputs are preserved in `docs/evidence/`.

| Command/check | Actual outcome |
| --- | --- |
| `bash scripts/bootstrap-cloud.sh` | Passed on rerun; Rust 1.100.0 and dx 0.7.10 confirmed, frozen npm install completed. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed. |
| `cargo test --workspace --locked` | **9 core tests passed**, none failed/skipped. Web binary/doc-test targets contain zero tests; browser coverage is separate. |
| `dx build --platform web --package kuro-web --release --locked` | Passed, with wasm-opt. About 1 MB of optimized WebAssembly before compression. This is not a startup-performance claim. |
| `npm run test:e2e` | **27 passed**, zero failures/skips in the final run: 11 workflows at desktop and phone sizes, plus 5 screenshot comparisons. |
| Accessibility | All 12 modules scanned on both workflow viewports by axe; no violations in final scans. This does not replace screen-reader or physical-device testing. |
| Visual comparison | Reviewed baselines match at 1440×900, 1180×820, 820×1180, 390×844 and 320×700. No horizontal page overflow at those widths. |
| `npm audit` | 0 reported vulnerabilities in the browser-test dependency tree. Cargo advisory audit not run. |
| Official PDF fetch | **Blocked**, actual HTTPS proxy 403. No content-completeness result claimed. |
| Package verification | ZIP CRC checked, SHA-256 manifest included. Extracted artifact launches and retains a newly created task after reload; see `artifact-smoke.json`. |

Browser checks cover task/agenda linkage, note literals and labels, refresh recovery, switching before debounce, accuracy, mistake/revision linkage, review completion, keyboard palette focus, themes after refresh, backup export and restore into a clean profile, malformed import rejection, aborted writes, stale-tab protection, invalid stored schema protection, navigation, console errors, module accessibility and overflow. Test data is isolated in temporary browser contexts; none is seeded into new accounts.

## Visual inspection

Inspected actual Chromium captures of all five target widths, plus desktop Tasks, Notes, Music and Settings. Original artwork and panel proportions survive the responsive changes. On phones navigation is a horizontally scrollable strip and forms become one column. Screenshots are in `docs/screenshots/`; reviewed comparison baselines are in `tests/e2e/visual.spec.js-snapshots/`.

These are viewport simulations. No real iPad, Pencil, Safari, iPhone hardware, screen-reader, native safe-area, OS text-scaling or long-session memory tests were run. Music is an explicitly disconnected visual study; it has no playback buttons pretending to work.

## Defects found and fixed

| Severity | Reproduction / actual behavior | Root cause and correction | Verification |
| --- | --- | --- | --- |
| P1 integrity | Edit a note and immediately choose New; previous draft could be replaced before debounce | Note switching now commits the current titled note before replacing the draft; an untitled nonempty draft blocks switching with a visible explanation | Regression passes on desktop and phone |
| P1 integrity boundary | Create enough valid long notes to exceed the 20 MB import bound; export previously had no matching bound | Export now checks final envelope size and refuses an unusable backup while retaining stored data | Ninth Rust test passes |
| P2 accessibility | Mobile CSS hides navigation numbers; accessible names differed by viewport | Explicit route labels keep identity stable in mobile and compact layouts | All phone workflows pass |
| P2 accessibility | Library/Exams unavailable screens went from h1 directly to h3 | Added a meaningful h2 module-status heading | All-module axe checks pass |
| P2 accessibility | Heading reveal opacity and dimmed nav numerals produced insufficient contrast during scanning; document had no language | Reveal changes position only, numbers retain contrast, and custom index declares English | Final axe scans pass |
| P2 rendering | Arrow controls appeared as missing-glyph boxes | Added licensed DejaVu symbol fallback | Re-inspected screenshots and visual comparisons pass |
| Verification defect | A test run reused an earlier development server | Browser tests now start their own release server on port 4174 and refuse reuse | Final 27-test run exercised hashed release assets |
| Setup blocker, resolved | dx attempted read-only caches and its embedded downloader rejected the proxy certificate | Writable XDG paths; exact tools installed through TLS-verified npm/curl or Cargo source installation. Verification was never disabled | Setup rerun and optimized release build pass |

Malformed backups, aborting transactions, stale tabs and malformed stored state were also exercised successfully. No known unresolved P0 defect was observed in the implemented workflows; this is not an exhaustive safety or production-readiness claim.

## Remaining limitations

- **Release-blocking scope gaps:** official syllabus/topic tracking, PDF library/viewer/annotations, handwritten notes, full calendar/recurrence, timers, exam engine/scoring, wallpaper imports, PWA caching and streaming integrations are unimplemented. The website does not satisfy the full acceptance list and must not be called production-ready.
- Streaming provider documentation and live accounts were inaccessible/unconfigured; neither real playback nor simulated-provider behavior was tested. No tokens or secrets were requested for unimplemented adapters.
- P2 capacity/recovery limitations: the database is one serialized record; very large histories are unprofiled, backups above 20 MB are refused, and browser eviction remains possible. Recovery drafts from a closed/lost tab session have no recovery-browser UI. Title notes and allow autosave/export before closing sessions. Backups contain committed notes only.
- P3 polish limitations: no-match task filters do not have a distinct explanatory empty state; compact/sidebar/page selection is not restored after reload. Provider choice on the disconnected music screen is a local UI selection, not a saved streaming preference.
- Source-section audits, schema-upgrade-failure tests, corrupted/scanned/large PDF tests, authentic exam-rule tests, actual full-disk tests, cloud sync interruption tests, deep performance stress, physical-device and native builds are **not run**. A simulated transaction abort tests failed-write behavior, not real disk exhaustion.
- SQLite and the complete platform adapter set are future work. Only the browser adapter runs. No public deployment or externally shareable preview URL was created; a locally served release and downloadable ZIP are provided.

## Platform status

| Target | Status |
| --- | --- |
| Web / Linux Chromium host | Release built, launched and browser tested; no complete PWA or Safari validation |
| iPadOS / iOS | Architecture path only; no build, packaging, installation, launch, device or signing validation |
| Android | Not implemented or tested |
| Windows / macOS / Linux native | Not implemented or tested; Linux host execution is not a Linux native-app build |

## Saved environment configuration and continuation

Saved `install_script`, `start_skill`, and required documentation/source domains in the environment draft. These saves are confirmed but **do not apply or publish runtime changes**. Review and save them in environment settings, then publish the environment. Fresh-task snapshot restoration is not independently verified.

The next priority is **M2's official-source gate**: apply access to `jeeadv.ac.in`, fetch the PDF, inspect every page, curate exact source-linked records and pass the coverage/ID audit before enabling topic tracking. See `docs/SYLLABUS.md`. The five visual screenshots are still needed before finalizing the reference-based design. Then complete planner and PDF milestones in controlled increments; do not treat the independent slices here as finished roadmap milestones.

Use the existing isolated `/workspace/Kuro` checkout. No worktree is needed. Activate `. scripts/env.sh`, run `bash scripts/check.sh`, and `npm run test:e2e`. The standalone release server uses port 4173; tests independently use 4174. No development continues after this checkpoint without a new active turn.
