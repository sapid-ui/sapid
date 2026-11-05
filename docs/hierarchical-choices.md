# TreeSelect and Cascader

Both widgets reuse the shared Tree navigation, selection and popup semantics.
Keys, labels and successful business values remain separate. Controlled sources
are read-only: callbacks propose replacement values and a refusal retains the
accepted display. TreeSelect supports single/multiple selection, cascade policy,
search and bounded declarations through the existing controller. A nonempty
search temporarily reveals the retained ancestor paths of matching rows. Clearing
the search restores the application's ordinary expansion source without an
automatic expansion callback or source write. Accepted trigger labels show leaf
labels; the complete key path remains the successful value, and the path example
displays that value separately.

Cascader defaults to columns and leaf selection. `change_on_select(true)` also
proposes branch paths; a branch remains open for further browsing, while choosing
a leaf completes the path. `with_columns(false)` selects the single-column Tree
layout at mount time. This is an explicit application choice, not an implicit
viewport observer or a separate flat breadcrumb navigation implementation.

Clear uses the same typed proposal and requests focus on the retained actual
trigger through the live controller. `clear_label` localizes the action and must
be a nonempty single line of at most 256 UTF-8 bytes. Invalid labels are rejected
before mounting. Read-only/disabled and current ownership are checked when the
action runs.

The TreeSelect guide has nine complete compiled examples: single/multiple,
cascade, search, controlled refusal, limits, passive rendering, geometry and
unavailable states. Cascader has five: full paths, controlled parent proposals,
compact/RTL layouts and a searchable TreeSelect alternative, local deferred child
publication, and unavailable/invalid states. The deferred example publishes
application-owned local data; it performs no implicit network or filesystem I/O.

Native CPU tests cover clear focus, retained trigger identity, parent selection
and compact/column navigation across four themes. Those tests do not establish
actual OS input or assistive technology acceptance. Frozen browser evidence and
current verification status belong to the shared-page remediation ledger.
