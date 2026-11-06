# Capability evidence review — 2026-11-06

This review reconciles the explicit capability matrix with current consumer
tests. Inventory completion and browser-page acceptance do not establish native
interaction or OS accessibility. The source baseline is Sapid `6999373` with
Glory `d8aa609`; subsequent changes in this review concern verification and
evidence bookkeeping.

## Current native consumers

The following command ran in `examples/native` with its normal Cargo target:

```text
cargo test --locked --all-features --test menu --test token_editor_layout --test input_actions --test combobox_docs --test extension_docs -- --test-threads=1
```

All 12 tests passed. The retained local log is
`examples/native/target/remediation-ci-evidence.log`.

| Inventory | Current source | Supported conclusion |
| --- | --- | --- |
| P4-01 Menu, P4-03 Menubar | `examples/native/tests/menu.rs` | Four-theme search/filtering, actual engine focus and editor identity, command/checkbox/radio activation and cleanup. |
| P4-05 Select | `examples/native/tests/input_actions.rs`, `combobox_docs.rs` | Four-theme hit/focus/geometry, clear actions, sizing, custom trigger identity and anchored empty message. |
| P4-08 MultiSelect | `examples/native/tests/token_editor_layout.rs` | Token/editor layout, wrapping, long labels, clear/loading alignment, disclosure focus and one open proposal. |
| P4-09 TagsInput/PillsInput | `examples/native/tests/token_editor_layout.rs` | Real editor identity, token wrapping/clearing and current focus. |
| P4-10 Command/Spotlight | `examples/native/tests/extension_docs.rs` | The real Spotlight source mounts and retires in four themes. Closed-shell lifecycle coverage does not establish popup search/activation, so native interaction remains model-only. |

These tests use the actual WindowApp engine with injected input, not OS window
interaction, system menus, stock clipboard, IME, physical touch or reader speech.
The matrix notes describe the finite tested subset; they do not claim exhaustive
upstream API parity. Existing AccessKit states are not promoted by layout tests.

P4-02 ContextMenu, P4-06 editable Combobox and P4-07 Autocomplete retain their
historical evidence and model-only native classification: this review has no
dedicated current consumer acceptance for those interaction contracts. Select's
shared Combobox path is not editable Combobox or Autocomplete acceptance.

## Placeholder and evidence references

The old blanket native placeholder rejection is obsolete. Current renderer
admission, patch/property-barrier and paint tests are in
`../glory/crates/apace/tests/placeholder.rs`; live selection/composition retention
is covered by
`placeholder_updates_leave_selection_preedit_and_empty_pointer_geometry_unchanged`
in `../glory/crates/apace/src/editing_tests.rs`. Placeholder metadata does not
become a label or expose a password value. Component-specific AccessKit states
and OS acceptance remain unchanged.

All matrix cells reference their canonical `inventory:<ID>` record. The 144
cells containing free-text documentation references now keep those references
under the corresponding inventory's `evidence.documentation`, preserving the
evidence while satisfying the generator's reference validation. The roadmap,
capability report and audit gallery are regenerated together; CI checks drift.

The native Drawer/Sheet, Tooltip and RichTextEditor rejection boundaries remain
explicit. Date/provider and event/recurrence model rows do not claim a host input
driver; the Rust documentation website does not claim a native site executable.
