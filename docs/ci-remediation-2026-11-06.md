# CI and showcase verification repair — 2026-11-06

The failed main runs were Sapid CI `42134940022`, dependency quality
`42134940031`, showcase deployment `42134939933`, and Glory semantic UI
`42134833265`. Compilation success did not establish browser acceptance or
deployment; the failures below were preserved before changing the gates.

| Failure | Correction |
| --- | --- |
| Browser runner read `target/web`, while CI served `target/verification/sapid/web` | CI now passes the same explicit artifact/output root to the browser runner and provenance recorder. |
| Showcase entry required `SAPID_BUILD_ROOT`, but deployment supplied only `BUILD_ROOT` | Deployment exports the artifact root consistently; standalone runs derive the normal owning workspace target through the shared helper. |
| Dependency regression loaded Playwright although it starts no browser | Browser loading is lazy; path/server/failed-evidence tests run without Playwright installed. |
| Export compilation assumed a redirected target and rewrote caller temporary/profile settings | The export fixture runs Cargo from its owning workspace, reads its actual target, and inherits caller configuration. Browser storage also inherits its normal default unless explicitly configured. |
| Glory audit assumed `CARGO_HOME` existed | Glory uses Cargo's normal home when the variable is unset; its audit policy and raw-warning retention remain intact. |
| Engine verification named an unavailable Apace baseline and the former namespaced Stylo graph | Glory's source ledger now matches the existing Cargo manifests: accessible Apace baseline, upstream Stylo 0.23 archive VCS/checksum verification, explicit separate Wry parser versions, and owning-workspace target resolution. No runtime dependency was switched by this repair. |

## Current documentation acceptance

The site replaced old single-playground pages with complete independent examples.
The deployment runner now requires all 32 current `showcase-remediation-*` batches,
including the nine batches underlying the 27-page priority audit. The P1 aggregate
does not run a duplicate copy of those nine suites. The 39-suite manifest also
retains site-wide navigation, themes/studio, exact copied Rust export compilation,
palette order, routing and real two-generation deployment-cache acceptance.

Component assertions now run against their current example sections:

- Buttons, variants, sizes, loading, identity, keyboard toolbars, clipboard and
  files: buttons/geometry batches.
- Editable/controlled fields, selection, search, tokens, trees, composition and
  current availability: inputs, form/data input, choice, combobox and selection.
- AppShell, Menubar/Menu, overlays, feedback, responsive layouts and navigation:
  shell, menus, composites, overlays, feedback, layout and navigation batches.
- Media, typography, data display, charts, dates, schedules and extensions:
  their respective complete documentation batches.
- Every route, regular-weight headings and mobile overflow: all-pages batch.

The site-shell suite still verifies the 181-link catalogue, actual SPA navigation
and search, real Button activation, exact compiled example source and clipboard
copy, source/properties/styles/capability tabs, four-theme tokens, 20 SSR pages,
dirty-editor identity/selection adoption, mobile navigation and friendly 404s.
Obsolete single-playground tests remain in source history/current files for
their historical reports; they are not used to infer acceptance of changed UI.

The gate remains fail-fast. It hashes served artifacts and test sources, records
pending suites, rejects missing/stale reports, nonzero drivers, caught assertion
`failure` fields and browser `errors`, and retains reports/logs/screenshots.
The eight runner regressions include a guard requiring every current documentation
batch exactly once and real HTTP rejection of stale/missing fixture bytes. Canonical inventory/capability/gallery drift is now checked
by CI before compilation.

The widget fixture driver accepts `SAPID_FIXTURE_BASE` for an owned loopback
HTTP origin while keeping 8794 as the CI default. Before any browser assertion,
every served artifact must match the recorded on-disk SHA-256. Only the fixture
origin is configured in the finite test sources; paths and assertions remain
unchanged. Reports retain both original and configured source hashes and the
origin, which the release provenance validator checks independently. This lets
local checks use an ephemeral server without terminating an older developer
server or accidentally testing its stale WASM/CSS/stream files. Frozen harness
checkouts include the shared browser path helper as well as the driver.

Current native evidence and intentionally unpromoted platform claims are recorded
in [the capability review](capability-evidence-review-2026-11-06.md). The standalone
stylesheet regression also found and fixed disabled/unstyled Sidebar and nested
Menubar trigger cursor leakage; all four focus-contrast variants and isolation
checks pass against the library stylesheet without showcase CSS masking it.
