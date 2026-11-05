# Alerts, decisions and placeholders

`alert(key).title("Saved").content("Changes are stored.")` is an inline message.
Info, Success, Warning and Danger choose typed theme colors independently of
`announcement(Announcement::{Off,Polite,Assertive})`. Polite uses Status,
Assertive uses Alert, and Off requests no live-region role. A title names the
region and the message describes it; an explicit name overrides title naming.
Each supplied title/message/close label is nonempty and bounded to 64 KiB.
Invalid reactive text preserves the last accepted value and can recover.

`closable(true)` adds a named real button. Local close hides the retained shell;
`visible(source)` is read-only and close only invokes `on_close` to propose it.
External visibility writes are silent. `fill(children)` adds actions or other
content. Classes append, explicit styles win, and unstyled removes theme colors.
Closing does not dispose application content; unmount the widget to retire it.

`alert_dialog(key).title("Remove draft?").description("This removes the draft.")`
uses the shared modal Layer and the distinct AlertDialog role. A name or title,
description, and safe cancel action are required. The default initial focus is
Cancel, even when editable content precedes it. Outside dismissal defaults off;
Escape can cancel, and both policies can be explicitly configured. Managed
initial/return focus use stable SemanticIds. The host owns focus containment,
Escape, outside routing and return focus.

Confirm and cancel labels are localizable. `confirm_enabled(source)` can disable
confirmation while leaving cancellation available; `confirm_variant` chooses its
Button styling. `on_confirm` and `on_cancel(AlertCancelReason)` run after local
acceptance and before the open-change proposal; Escape also invokes `on_dismiss`
after typed cancellation. Callbacks recheck Owner liveness. Controlled refusal
keeps the panel open, and external source updates do not emit callbacks. Content
can unmount or remain mounted with its draft and node identities. Applications
own validation, asynchronous work and destructive operations.

`floating_dialog(key).title("Background sync")` is a separate, centered,
nonmodal notice with a Preserve focus policy. It neither moves focus on opening
nor traps Tab. Outside dismissal defaults off; background controls remain usable.
It accepts interactive content and the same controlled/local/content policies.
Its API does not offer managed initial/return focus. Browser-native popover
behavior remains part of the shared Layer implementation. This is a notice,
without movable/resizable desktop-window behavior.

`skeleton().width(Length::Px(220.0)).height(20.0)` creates a fixed placeholder;
`circle(40.0)` sets a circular box. Width is explicit pixels or percent, height
is positive finite pixels, and radius is finite/nonnegative. Reactive invalid
dimensions retain the accepted geometry. Its finite rectangular Graphic is explicitly decorative; the parent owns busy
metadata and may have a loading-region name. `fill(children)` retains genuine
content and intrinsic height unless explicitly sized; loading makes that content
transparent/inert without replacing its editor, value or Owner. A standalone
`loading(false)` hides the placeholder. The default host-owned 1500 ms opacity
pulse honors `animated(false)` and reduced motion without a widget timer or
per-frame Cage writes. Unstyled retains essential geometry and explicit styles.
Native Graphics require paint. Current content, icon-close and empty-state
composition examples are documented in [Feedback](feedback.md).

Focused model checks pass: Alert 5, AlertDialog 5, FloatingDialog 3, Skeleton 4,
plus existing overlay regressions 6. Render validates AlertDialog's managed
modal/safe-action policy and rollback; HTML's full-shape differential oracle
includes it. Native AccessKit exports its distinct role/name/description in a
model test; this does not implement native top layers.

`examples/native/tests/alerts.rs` passes three tests, including four-theme real
inline-message and Skeleton geometry/identity, controlled refusal, and explicit
Layer rejection before a partial component remains. The browser suite
`examples/web/alerts-acceptance.js` passes eight four-theme CSR/adopted-SSR
cases on Chromium 148.0.7778.96. It checks actual live-region AX properties,
decision role/description, safe focus, modal and nonmodal Tab, controlled refusal,
Escape, retained drafts, decorative Skeleton identity and mobile overflow.
Reports are `D:/BuildCache/sapid/alerts-browser.json`, `alerts-native-final.log`
and `alert-ax-native.log`. Browser and tree evidence does not establish spoken
screen-reader output or OS input-method acceptance.


Native Dialog/AlertDialog Manage and nonmodal Dialog Preserve are now supported
by WindowApp's finite top-layer driver. The four-theme AlertDialog consumer
checks safe initial Cancel focus, actual bounds/CPU pixels, controlled Confirm
refusal, stable panel identity and the declared trigger's return focus. The
FloatingDialog consumer checks real layout, unchanged background focus and
Owner cleanup. `examples/native/tests/alerts.rs` passes all four tests; combined
with ColorInput consumers, `D:/BuildCache/sapid/native-new-layer-consumers-final.log`
records seven passing tests. The engine provenance and remaining per-facet
refusals are documented in `../glory/docs/native-top-layers.md`.


The 2026-11-05 showcase remediation adds default compact vector Alert closes,
contrast-aware filled text, busy Notification icons and retained content
Skeletons. Current focused counts are Alert 7, Skeleton 5 and Notification docs 2;
`examples/native/tests/feedback_docs.rs` has six additional real pixel/geometry
checks. The final 64-case four-theme/two-width browser report is
`examples/web/target/remediation/feedback-final.json`, resource
`assets-56f459bdcd14dd1e6252`, errors empty. Earlier paths above remain historical
acceptance records; current build outputs use the owning workspace's target.


FloatingDialog now accepts `position(ViewportPlacement)` for finite host-owned
viewport x/y/padding and constraints, keeping its Preserve focus policy. Its
public page starts closed and has three standalone position, controlled
background-editor and content-lifetime examples. Ordinary Dialog/Drawer/Popover/
FloatingDialog headers use compact vector closes; AlertDialog retains safe
visible Cancel behavior. See [Overlay pages](overlay-pages.md).
