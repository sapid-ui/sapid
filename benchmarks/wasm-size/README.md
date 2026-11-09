# Sapid WASM consumer matrix

Run from the Sapid repository with Glory, Apace, Takia and Wingy siblings present.
Requirements: the declared Rust toolchain with `wasm32-unknown-unknown`,
wasm-bindgen **0.2.130**, Node **22+**, and Playwright with Chromium for browser checks.
No tool is installed by these runners. Use the existing locked deployment tools
or set `WASM_BINDGEN`, `SAPID_PLAYWRIGHT` and optionally `BROWSER_EXECUTABLE`.

```powershell
node --test benchmarks/wasm-size/test.cjs
node benchmarks/wasm-size/run.cjs --label baseline
node benchmarks/wasm-size/browser.cjs <absolute-run-directory>
# Faster subset, with the same input validation:
node benchmarks/wasm-size/run.cjs --label quick --only empty,glory-text,sapid-button
node benchmarks/wasm-size/run.cjs --label candidate --compare <absolute-baseline-report.json>
```

The runner prints its absolute report path. The evidence directory is
`verification/sapid-wasm-size/runs/<label>` under this workspace's actual
`cargo metadata` target_directory. Existing labels are rejected to preserve
evidence. Cargo home, target and compiler temporary paths inherit the caller.
The runner does not copy or redirect build sources and does not alter environment
flags. Reports preserve relevant release overrides; comparisons reject different
toolchains, profile settings, targets, locks, partial baselines or missing cases.

There are 17 cases: empty, Glory text, unused Sapid, Sapid button, six unused and
six used optional families, and all-unused. Each used family mounts a concrete
widget. Browser evidence validates the measured WASM/JS hashes before mounting,
exercises appropriate interaction and verifies DOM cleanup after unmounting.

The comparison defaults to a 5% growth budget for each WASM/JS raw, Brotli and
gzip metric; use `--max-growth <percent>` to choose a reviewed budget. The pipeline
is Rust release plus wasm-bindgen, not the historical Binaryen-optimized pipeline.
Structural CSS is served for the browser probe and excluded from WASM/JS totals.
One representative workload does not measure every widget in a feature family.

Record source revisions and preserve dirty changes with the report. Source
content digests detect changes during measurement but are not source archives.
These probes measure transfer size, not runtime responsiveness or allocator use.
