# Miscellaneous showcase pages

The ten pages below have individual compiled primary examples and 34 independent,
complete-source sections. Each section has a stable anchor and matching contents
entry. Their controls operate on local Cage state, actual editors, focus scopes,
portal destinations and scroll viewports. Documentation headings use weight 400.

| Page | Examples and behavior |
| --- | --- |
| Box | Independent padding, border, background and radius; real action composition; reactive appearance on the retained box. Raw HTML/CSS templates remain a separate display layer. |
| Collapse | Controlled acceptance/refusal, content-driven height, changing content, duration/animation control, host completion and KeepMounted editor retention. |
| Divider | Solid/dashed/dotted lines, three horizontal label positions, vertical boundaries, long reactive labels. |
| FocusTrap | A real shipping form with requested initial address focus, Tab containment and restoration; independently configured policies and nested scopes. |
| Marquee | Text-only measured loops, two directions, cycle duration, pause, animated/static comparison and reduced-motion behavior. |
| Paper | Independent border/shadow/spacing, nested surfaces and composition with Card. |
| Portal | Explicit registered destinations, moving the same editor, destination loss/recovery and enabled/parked content. |
| ScrollArea | Both axes, five scrollbar policies, explicit timer, scrollbar dimensions/offsets, nested containment and live position commands. |
| Toolbar | Compact icon actions with accessible names, groups, pressed state, disabled-item navigation, RTL/vertical navigation and explicit OverflowList composition. |
| VisuallyHidden | Plain hidden names, an actual described_by relationship and a separate focus-revealed skip link. |

## Divider

`separator().variant(SeparatorVariant::{Solid,Dashed,Dotted})` uses independent
typed border declarations for patterned lines. A plain separator retains its
separator role. `.label(text)` creates a generic layout containing ordinary text
and decorative lines: passive separator nodes cannot have children. It does not
invent an interactive separator role. `.label_position(SeparatorLabelPosition::
{Start,Center,End})` controls the line arrangement. Labels are horizontal only,
nonempty, at most 256 UTF-8 bytes and contain no NUL or line separators. Invalid
reactive proposals keep the last accepted label; later valid proposals recover
without replacing nodes. Theme removal retains structure; explicit styles still
apply. Positive line thickness, length and orientation retain their existing
validation. There is no implicit icon-content or vertical-label API.

## Collapse completion

`.on_transition_end(|open| ...)` observes the current accepted height presentation.
It opts into a 180 ms height transition when none was configured. Explicit
duration, zero duration and `.animated(false)` remain authoritative. The initial
presentation is silent; each current terminal revision notifies at most once.
Stale and duplicate host completions do not call application code. Reversing a
transition reports only the current completed revision. A duration or policy
change can settle a new presentation revision without changing logical openness.

The browser and Apace supply actual completion events. SSR and the plain memory
model do not manufacture them. A closed panel becomes logically hidden and inert
immediately while its clipped closing pixels can remain until completion.
KeepMounted retains the editor and draft; other content policies have their own
ownership contract. This is intrinsic-height animation; horizontal collapse is
not a delivered capability.

## Focus, portal and hidden text

FocusScope is nonmodal: it does not make content outside it inert. The form builds
the actual FieldControl identity into `FocusScopeConfig.initial_focus` before
activation. Closing a scope returns focus to its still-live initiating button.
The containing application supplies visible layout and activation state.

Portal uses an explicit Owner-local context and named targets. It relocates a
retained range; it does not render a second editor. Losing a target parks the
content until registration returns. A portal alone supplies no overlay,
positioning, modal behavior or global body destination.

VisuallyHidden has plain text, zero layout extent and accessible content. It
does not accept focusable children. The description example deliberately uses
the standard Glory button's explicit Semantics contract to bind `described_by`
to the hidden node. The visible action name remains separate. A skip link uses
an ordinary Anchor that reveals itself on focus; hiding an interactive element
permanently is not an accessible use of VisuallyHidden.

## Scrolling, toolbar and motion boundaries

ScrollArea retains the real viewport. Never hides scrollbar presentation without
disabling scrolling. Hover and Scroll require an explicit delay factory for
nonzero hide delay. The complete example includes a cancellable 300 ms browser
timer; its nonbrowser branch completes immediately and does not claim an OS clock.
Native applications supply their own local clock. Position commands use the
current Owner-bound ScrollHandle and actual metrics, rather than replacing DOM.
Native scrollbar appearance and OS touch behavior remain host-owned.

Toolbar owns roving keyboard navigation across current enabled buttons. Ordinary
groups wrap visually; OverflowList is an explicit separate composition. Icon-only
buttons retain their full accessible names. No clipboard or editing service is
implicitly installed by a toolbar example.

Marquee repeats plain text visually and exposes one accessible original. Period
is the duration of a full measured cycle, not a fixed pixels-per-second speed.
Pause retains the current animation; disabling animation produces static content.
Browser reduced-motion preference suppresses the loop. The browser and Apace implement finite translation motion. Native uses its
explicit motion preference and document clock; OS preference synchronization
is a separate integration.

## Verification scope

The public detail sources mount and retire in all four built-in themes with
matching navigation. Native consumer tests import the actual public examples and
check narrow Divider geometry/rasterization, physical Portal editor retention,
requested shipping focus/restoration, real ScrollHandle position changes and
zero-extent description relationships. Existing native tests cover Collapse
completion/reversal, portal selection, scrollbar commands and Toolbar retention.
Browser acceptance covers both desktop and phone widths in all four themes.
Protocol, CPU layout and browser accessibility-tree checks do not establish OS
input, screen-reader speech or mobile-device acceptance.
