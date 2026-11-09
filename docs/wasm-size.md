# WASM size probes

The active consumer matrix belongs to Sapid:
[`benchmarks/wasm-size`](../benchmarks/wasm-size/README.md). It has 17 cases:
an empty WASM module, Glory text, unused Sapid, a reactive Sapid button,
each of the six optional families both enabled-but-unused and actually mounted,
and all six unused families together.

Builds inherit the caller environment and Cargo configuration. The runner reads
`cargo metadata --no-deps --format-version 1` in the benchmark workspace and puts
evidence beneath its returned `target_directory`. It does not redirect Cargo
home, target, TEMP/TMP or install a shared cache root.

```powershell
node benchmarks/wasm-size/run.cjs --label baseline
# Use the absolute report directory printed by the runner:
node benchmarks/wasm-size/browser.cjs <run-directory>
node benchmarks/wasm-size/run.cjs --label candidate --compare <baseline-report.json>
```

The current pipeline measures Rust release plus wasm-bindgen output, without a
separate Binaryen pass. Reports record source commits and content digests for all
five sibling repositories, dirty status, compiler/tool versions, release flags,
lock hash, WASM and transitively imported JavaScript hashes, and raw/Brotli/gzip
sizes. A changed source set during a run fails the measurement. Preserve the
recorded source revisions and any dirty changes to reproduce that run.

Browser checks mount every selected case, exercise the reactive button and
editable inputs, and unmount to check cleanup. External structural/theme CSS,
fonts and images are separate assets. These are transport-size probes; they do
not measure allocator usage, frame time or complete widget-family parity.

## Historical measurements

The former 36-application Glory/Sapid matrix and its optimized results are
historical evidence. Glory now maintains a Glory-only matrix; its current runner
is not the Sapid consumer entry point. The values below used a different pipeline
and must not be compared directly with a new active run.

Unused Sapid and all six unused optional features produce byte-identical
WASM to Glory text in the measured configuration. Using a widget retains its
reachable behavior and dependencies, not every exported widget. A used
feature example is a specific workload, not the size of an entire family.

The reactive Button application falls from 1,029,679 to 995,151 B
(34,528 B / 3.35% saved), and from 319,564 to 310,071 B with Brotli
(9,493 B / 2.97% saved). This includes the shared Glory mapper reduction.
The same button with all six unused features is 995,411 B / 310,379 B
Brotli.

The first optimization isolates optional toolbar hooks from ordinary Button
construction, so standalone buttons no longer retain collection and roving
focus behavior. Public behavior and default features are preserved; toolbar
setup uses one boxed private hook. Existing action, button and toolbar tests,
the full Sapid check script, and browser fixtures validate the change.
