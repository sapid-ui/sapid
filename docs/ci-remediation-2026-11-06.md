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

## Remaining fixture assumptions found by the complete browser scan

The classic stream handoff had a real runtime defect: it validated HTML format
19 but emitted version-14 root markers at End. Glory now derives both markers
from the validated End frame. Current positive protocol fixtures use format 19;
old-version rejection and failure-prefix assertions remain in place.

The current fixture checks also reflect the implemented public contracts:

- Rich editor commands are scoped to their owning editor shell, rather than
  assuming the toolbar is inside the content wrapper. Both original rich suites
  retain typing, UTF-16 selection, history, IME, refusal and retirement checks.
- The header close control is the first enabled control under the documented
  default managed-focus policy. Tests verify that default, then exercise real
  editor focus, Tab/Escape, nested/cross-App scopes and return focus. They do not
  change the component policy to satisfy an older DOM-order assumption.
- Stepper checks its full accessible label separately from the number glyph;
  default line Tabs check their themed 2px underline; Sidebar measures its 72px
  outer border box rather than the 71px content box.
- Scroller buttons are found by accessible labels, not vector-glyph text. The
  color plane checks four bounded continuous-gradient layers and all four real
  keyboard sliders instead of expecting the retired 256-cell raster grid.
- The legacy clamping RangeSlider fixture explicitly selects `RangeCollision::Clamp`.
  The public default remains Push; the documentation suites separately exercise
  Push/Hold/Clamp. The unstyled date fixture declares its intended 240px width,
  and the Chinese time fixture declares its segment and period labels.
- Agenda uses native list/listitem semantics and level-3 date headings. Its
  all-day interval is checked on the correct single date rather than requiring
  the old raw timestamp caption. The MiniCalendar boundary checks its range
  navigator, not controls belonging to the full Calendar.
- Lazy tree publication is an explicit application source update while the tree
  remains open and focused. Pointer submission of the multiple-choice form first
  closes its covering listbox with Escape. No forced pointer click bypasses a
  top-layer popup.

The scan's original failures are retained under `target/remediation`, including
`ci-first-full-fixture-results.json`, `ci-first-full-fixture-failures.json` and
`ci-fixture-browser-owned.log`. A changed source or artifact invalidates a run;
passing evidence must come from a fresh immutable run after the repairs.
