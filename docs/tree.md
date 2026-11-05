# Tree and FileTree

## File-tree guide

The public usage example is a project file tree with branch/file graphics,
connecting guides, real keyboard/pointer selection and readable selected-key
feedback. Controlled refusal, owner switches and bulk data probes remain
acceptance concerns rather than the primary reader flow.

All eight detail sections now include the complete Rust module actually compiled
for its preview, including its data and helpers: controlled expansion/selection,
bulk expansion, cascading and independent checks, application-owned deferred
children, external search, drag/drop proposals and passive custom labels.
The deferred example publishes local sample data and performs no implicit I/O.
The existing tree controller, finite model and shared host navigation remain the
same; these guides do not claim arbitrary upstream hooks or full API compatibility.

`tree(key).nodes(source)` renders real Tree/TreeItem semantics with independent
focus, expansion and selection. `TreeNode` supplies a stable key, explicit parent,
plain-text label, Leaf/Branch/Lazy kind and disabled flag. Current data is bounded
to 2048 nodes, 64 levels, 4096 bytes per label and 2 MiB total text. Keys are
1..64 semantic ASCII bytes; duplicates, missing/unloaded parents and cycles fail
before custom factories run. Invalid external data preserves the last valid UI.
Rows follow source order within each sibling set.

```rust
use sapid::{TreeKind, TreeNode, tree};
use std::collections::BTreeSet;

let files = tree("project")
    .nodes(vec![
        TreeNode::new("src", "Source").kind(TreeKind::Branch),
        TreeNode::new("main", "main.rs").parent("src"),
    ])
    .default_expanded(BTreeSet::from(["src".into()]));
```

`expanded(source)` and `selected(source)` are read-only controlled sets. Callbacks
propose changes; local defaults apply only without a controlled source. Proposals
within a host tick compose, then reconcile after the whole tick. A refused
expansion stays closed for following navigation in that tick. Explicit future
keys may remain in the application's bounded sets; the component does not keep
visited keys. Single selection accepts at most one key; Multiple toggles keys
independently, and None omits selected metadata. Optional selection-follows-focus
is limited to Single mode. Focus alone otherwise leaves selection unchanged.

Up/Down skip disabled visible items. Home/End move to the first/last usable item.
Right opens a branch, then enters its first usable child; Left closes an open
branch or returns to its parent. RTL reverses physical Left/Right. Space selects;
Enter selects and emits activation. Typeahead uses current visible labels, a
64-byte query and explicit host timestamps with a 700 ms timeout. Composition,
editing controls and command chords keep their native policy. A focused item is
revealed through the shared nearest/instant ScrollIntoView action. It neither
visits an unmounted business collection nor borrows virtual-list navigation.

Collapsed descendants unmount their short Owners. Keyed source reorder preserves
mounted item and custom-editor identities, drafts and selection. Closing/removing
the focused branch repairs focus to a live ancestor or first usable item. The
registry retains current mounted NodeIds only; root disposal clears refs,
callbacks, current cursor/search and pending focus. There is no ambient timer,
network request or history cache.

A Lazy node emits `on_load_children(key)` when opened. Applications own their
Resource, cancellation, errors and retry policy, then replace that node with
Branch and publish its children. Repeated Right on an already open Lazy node
does not start another request; closing and reopening explicitly retries. Empty
loaded branches remain distinguishable from unloaded nodes.

`expanded_tree_branches(nodes)` returns the current finite branch-key set for
explicit Expand all/Collapse all controls. `filter_tree_nodes(nodes, query)`
matches labels case-insensitively and retains every ancestor needed to reach a
match. Both validate the same budgets before deriving data and keep control of
the accepted source in the application.

`on_drag_drop(callback)` enables real pointer drag proposals. The drop is typed
as `Before`, `Inside` or `After` from the target row geometry; `allow_drop`
can reject a proposal before publication. `move_tree_node` validates the same
operation for controlled application state, rejects self/descendant cycles and
preserves sibling ordering. No global drag registry or hidden source copy is
retained. Drag rows use native pointer hit targets without a transient focus
outline or a composed color-strip image.

`render_label(factory)` receives reactive current TreeNode data and may render
ordinary text, code or controls. The node's short plain-text label remains its
explicit accessible name. Nested controls preserve their own editing/activation
policy, while a real enclosing Fieldset propagates the tree's disabled state.
Theme, size, direction, class/style, unstyled, name, enabled and node_ref follow
ordinary builders. An optional height creates an internal scroll viewport. This
finite Tree mounts all currently expanded nodes; it does not claim windowing of
an arbitrarily large source.

`file_tree(key, entries)` is a pure FileEntry adapter. Entries use explicit keys,
parents and bounded relative display paths; aliases, duplicate paths and parent
path mismatches are rejected. Labels use the final path segment and optional
kind markers. No filesystem lookup, file permission or upload is implied.

HTML emits one-based `aria-level`, `aria-posinset` and `aria-setsize`, with
expanded/selected/multiselectable metadata. Native AccessKit lowers the live
preorder into TreeItem/Group ancestry, uses zero-based levels/positions and puts
known sibling size on its container. Synthetic groups retain document-local IDs
only while live and receive new IDs after retirement. It exports the mounted
semantic tree, never a data source.

Evidence:

- `src/tree_model.rs`: eight tests for independent budgets, explicit parents,
  visible coordinates, lazy replacement, sentinel admission, FileTree paths,
  branch expansion, ancestor-preserving filtering and validated reparenting.
- `tests/trees.rs`: eleven component cases cover four-theme SSR, same-tick accepted
  and refused control, selection/focus separation, lazy data, keyed editors,
  nested activation, generic observer ordering, disabled Fieldset and cleanup.
  `D:/BuildCache/sapid/tree-model-tests.log`, `tree-components.log`.
- Glory `crates/render/tests/tree.rs`: three protocol/coordinate/rollback/key
  tests; `crates/html/src/tree_tests.rs`: full-shape field/reset differential
  oracle. `tree-render-tests.log`, `tree-html-tests.log` in the D: build root.
- Glory native Tree key/AX tests passed two cases, with actual queued keys,
  editor/modifier isolation, consumer hierarchy/counts and nonrecycled groups:
  `tree-native-shared-tests.log`.
- `examples/native/tests/trees.rs`: two real four-theme engine tests passed
  geometry, indentation, selected CPU pixels, custom-editor focus/identity,
  physical scroll reveal, lazy data and Owner retirement:
  `D:/BuildCache/sapid/tree-native-components.log`.
- The original twelve-group browser suite passed fixed Web29 FORMAT9 on
  Chromium 148.0.7778.96 with errors `[]`: four-theme CSR/hydration, real keys/AX,
  dirty editor adoption, same-tick open/navigation/select, controlled refusal,
  lazy data, multiple selection, RTL, mobile/touch and cleanup.
  `D:/BuildCache/sapid/tree-browser-web29.json`. The strengthened original suite also passed all twelve groups against fixed
  Web30, including genuine disabled propagation to nested editors through the
  enclosing Fieldset: `D:/BuildCache/sapid/tree-browser-web30.json`.
- `scripts/tree-showcase-browser.cjs`: seven Chromium checks cover all eight
  component-name sections, bulk expansion, both checked policies, lazy loading,
  filtering, real pointer drop and custom labels/connecting lines with zero page
  errors: `D:/BuildCache/sapid/tree-showcase-browser.json`.

These checks do not establish OS IME or screen-reader announcement acceptance.
