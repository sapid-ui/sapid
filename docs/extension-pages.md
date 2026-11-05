# Extension page examples

The seven pages below mount independent Rust examples. Their source panels
include every helper and compile the same `demo(theme)` function as the preview.
The active palette/mode supplies presentation; page and section headings use
regular weight. There is no extension-wide global state or JavaScript UI layer.

| Page | Primary example | Independent sections |
| --- | --- | --- |
| CodeHighlight | Complete Rust welcome function | Languages, theme/unstyled, numbered/highlighted lines, local long-line scrolling, multiple files/copy feedback |
| JsonViewer | Project configuration tree | Controlled expansion/refusal, escaped JSON pointers/keyboard, complete-source copying, search/data replacement |
| Notifications system | Edit a draft title and save it | Bounded FIFO stack, update/dismiss/Undo, viewport placement, explicit timeout/pause/resume, inline loading |
| Spotlight | Project command palette | Keyword/empty search, groups/disabled actions, scoped shortcut, controlled opening/search |
| Lightbox | Two-photo thumbnail gallery | Image navigation, stepped zoom/fit, caption replacement, controlled view/close refusal |
| NavigationProgress | Load a project page and publish it after completion | Operation replacement, cancellation, determinate completion, latest-operation concurrency |
| Modals manager | Edit a project name or confirm local deletion | Editor→confirmation queue, live metadata update, queued/current closing, focus restoration |

Sources live in `examples/web/src/dedicated_demos` and
`examples/web/src/showcase_extension_docs`. The source catalogue also exposes the
actual model, request, token and handle types used by these examples.

## Code and JSON

Code highlighting supports Plain, Rust and JSON lexical presentation. It does
not compile source or validate malformed JSON. Numbers occupy an inert gutter;
copying uses the whole accepted source without the gutter or token decoration.
`start_line` changes displayed numbering and `highlighted_lines` remains relative
and one-based. Long lines scroll inside the code viewport. Multiple files are
application-selected sources, rather than a hidden file loader or filesystem API.

JsonViewer's tree mode uses the shared Tree keyboard/focus controller. Controlled
`expanded` input contains JSON pointers: the empty string is the root, `~1` is
slash and `~0` is tilde. Refusing a proposal keeps accepted paths unchanged.
Search and current source replacement are separate inputs. Copying is independent
of collapsed branches and never serializes the visible DOM as JSON.

Copy feedback changes only after a host clipboard outcome. Browser acceptance
injects an isolated clipboard provider and does not write the user's clipboard.
Native clipboard behavior remains subject to its explicit host service; code
display alone does not grant clipboard access or native text-range selection.

## Local notifications and commands

Each Toaster has one Owner-local bounded queue. Showing an existing key replaces
that request lifetime; updating its token changes title/message in the same row.
Dismissing a visible row promotes a queued request. Actions remove the request
before invoking application code. The primary save action includes the current
draft title, stores a separate local saved value and produces a real notification, without counters or test controls.

Ordinary document placement is the default. The position section composes one
Toaster with Affix; the component installs no global notification singleton.
The timeout section explicitly supplies a monotonic browser Performance clock
and an Owner-cancellable timer. Its cold SSR/native preview installs a pending
clock and does not claim native wall-clock expiration. Hover/focus/manual pause
preserve remaining time and queued deadlines start only after promotion.

Inline Notification loading is independent of Toaster. It temporarily displays
a loader and blocks dismissal until completion. Toaster `update` changes
title/message; it does not expose an invented queued loading property.

Spotlight executes project commands from a real modal/input/listbox. Static
matching includes label, description and keywords; grouped disabled actions
remain unavailable. Unmatched search produces a visible empty result. The
shortcut section registers Ctrl/Cmd+K on one explicitly supplied physical
ancestor and retires it with its Owner. It installs no global listener, and
editors/composition retain the host's shortcut guards. Open and search callbacks
are independent proposals that an application may refuse.

## Images, asynchronous navigation and modal requests

Lightbox `show_trigger(false)` lets an application thumbnail gallery own its
openers, while retaining the shared modal Layer's focus and restoration rules.
The application supplies stable image keys, nonempty alternatives and captions.
Previous/Next and keyboard navigation obey boundaries. Zoom uses finite 100%,
200%, 300% and 400% steps inside the image viewport; Fit resets it. Pinch gestures
are not implied. The controlled example refuses the first close once, then
accepts the next proposal so an exit is always available.

The two thumbnail assets are local fictional samples from the built-in image
generator; their complete prompts and provenance are in
[generated-showcase-media.json](generated-showcase-media.json). They make no
claim about a real person or location. Native image transport is an independent
explicit host capability, not a grant inferred from a URL in an example.

NavigationProgress owns one current token. The primary example supplies a
cancellable local asynchronous loader, reports 25% then 70%, and replaces the
visible project page only when `finish` accepts its current token. Starting
another page cancels the prior task and revokes its token. The source installs
no network or route interceptor. Cancellation of a bar token alone does not
cancel an arbitrary application future. Independent simultaneous operations need
independent controllers or an explicit application aggregation policy.

ModalsManager's primary form publishes a project name only on Save; confirmation
changes only the example's local state. FIFO requests construct content only
when current. Metadata updates retain the current input and draft. A request can
close a queued confirmation without closing its current editor; close-all removes
the admitted sequence before callbacks. The shared host contains modal focus,
gives confirmations safe Cancel focus and restores the original live opener
after the final request. These are bounded local request lifetimes, not global
dialogs, completed-operation history or a separate modal controller.

The remediation ledger records the completed model, native and browser evidence.
Browser/CPU semantic acceptance does not establish OS input, screen readers,
mobile pinch gestures or stock platform clipboard services.
