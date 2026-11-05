# Feedback content and loading placeholders

Alert, EmptyState and Skeleton use ordinary Glory/Sapid Widgets and share the
four theme presets. Their showcase pages contain complete compiled primary and
section sources, with helpers included and navigation matched to each example.

## Alert

Severity and `AlertVariant::{Light,Filled,Outline,Default}` are presentation;
`Announcement::{Off,Polite,Assertive}` separately controls announcements.
Optional icons and long/untitled content wrap in the available width. Close is
a centered 16 px vector inside a 24 px button with a localized, reactive
`close_label`; it proposes dismissal through `on_close`. A controlled visible
source can refuse dismissal. Filled foregrounds account for background alpha
and select sufficient contrast. No browser-specific SVG/string injection is used.

## EmptyState

Title/description, illustration, body and actions are explicit slots. Absent
slots do not leave empty layout wrappers or phantom gaps. Actions wrap on narrow
screens. Empty-state selection belongs to application data: the search example
filters three real local project names and reset changes that query. No automatic
loading/alert announcement or data-fetch policy is inferred.

## Skeleton

`loading(Vary<bool>)` defaults true; `animated(Vary<bool>)` defaults true.
Without children, the standalone placeholder defaults to 16 px height and hides
when finished. Explicit `height` and `circle` retain their dimension rules.
`fill(children)` keeps real content mounted, derives intrinsic height unless
explicitly overridden, and overlays a clipped placeholder filling the entire
box. During loading the content remains in layout but is transparent and inert;
editor identity, value and owner state survive. This does not suspend tasks or
turn values into disabled form controls. The loading switch belongs outside the
inert content. Release only makes content available; it does not steal focus.

The placeholder uses a finite stretched rectangular Graphic and a host-owned
1500 ms pulse. Rounded/circular clipping applies to actual pixels. Pausing,
reduced motion and ending loading clear active motion without per-frame
reactive writes. Native tests check intrinsic sizing, right-edge coverage,
circle clipping, focus refusal during loading, retained physical editor identity
and static reduced-motion pixel caching. These are CPU/host protocol checks;
OS input and screen-reader speech remain separate acceptance.

Related contracts are in [Spinner](spinner.md), [Notifications](notifications.md)
and [Progress](progress.md). Focused runtime checks live in `tests/alerts.rs`,
`tests/display.rs`, `tests/skeleton.rs` and `tests/notification_docs.rs`.
