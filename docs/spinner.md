# Spinner and Loader

`spinner()` and `loader()` construct the same loading indicator. The
default accessible name is "Loading"; `name(text)` supplies a localized name.
A named spinner is a passive status with busy metadata. `decorative()` removes
the name, status and busy announcement, preserving a decorative Graphic rather
than duplicating its containing region's announcement. No button, range or
keyboard behavior is implied.

The default Oval has a bounded three-quarter circular path. Animation is a finite
host-owned rotation, not a component task, per-frame reactive write or timer.
The browser honors prefers-reduced-motion dynamically. Native hosts explicitly
configure a motion preference; unknown defaults to Reduce (a visible static
shape). Native OS preference synchronization is not provided automatically.

Sizes use the shared ExtraSmall/Small/Medium/Large/ExtraLarge labels with
12/16/24/32/40 pixel diameters. Explicit diameter is 8..=128 pixels, stroke
thickness is 1..=64 and at most half the diameter, and rotation periods are
16..=60000 milliseconds. Names are nonempty and at most 256 UTF-8 bytes.
`visible(Vary<bool>)` hides the retained indicator. `animated(Vary<bool>)`
can opt out of rotation; it cannot override a reduced-motion host preference.
Theme, unstyled, custom class/style and node references follow the other passive
components. Unstyled retains visible path content and essential geometry.

`examples/web/src/spinner_form.rs` supplies named, decorative and unstyled examples
with controlled animation and visibility. `tests/spinner.rs` has four tests for
all four themes, all five sizes, initial budgets, SSR output, controlled invalid
name recovery, retained identity and Owner cleanup. No frame observers or
component tasks are installed.

The native acceptance test in `examples/native/tests/spinner.rs` passes for all
four themes. It compares real raster pixels before/after a quarter turn, checks
that the application records no reactive source writes, verifies reduced-motion
pixel cache stability and the static default, and exercises visibility/cleanup.
The result is `D:/BuildCache/sapid/spinner-native.log`. Glory's three focused
motion tests additionally prove ancestor clipping, half opacity, coalesced host
wakes and restoration after Suspend/Resume. Shared render/UI/HTML/SSR tests and
strict Clippy pass; `D:/BuildCache/sapid/spinner-protocol.log` records the full
shared regression.

`examples/web/spinner-acceptance.js` passes all nine scenarios on unified Web9
with Chromium 148.0.7778.96: four themes in CSR/hydration plus classic streaming.
The actual report is `D:/BuildCache/sapid/spinner-browser.json` (nine passed,
no errors). It checks Chrome's real status/live/busy accessibility metadata,
advancing transform matrices with zero per-frame DOM writes, runtime
reduced-motion stop/resume, controlled animation removal/default clearing,
identity through visibility changes, narrow layouts and cleanup. The stream
case applies content animation attributes and runs before WASM, then releases
its controller on End. Theme screenshots are
`D:/BuildCache/sapid/spinner-{blue|green}-{light|dark}-hydrate.png`; the green-dark
output was visually inspected. These checks do not claim actual screen-reader
announcements or native OS motion-preference synchronization.

## Loader kinds and composition (2026-11-05)

`kind(LoaderType::{Oval,Bars,Dots})` selects one of three finite presentations;
Oval remains the default. Bars and Dots use three retained passive Graphics with
phase-offset opacity pulses. `color(Color)` overrides their theme accent.
`animated(Vary<bool>)` pauses every kind without replacing its graphics. Pulse
periods use the same 16..=60000 ms budget as rotation; native and web hosts own
frames and reduction policy. No per-frame Cage, callback or timer is installed.

The Loader page includes independently compiled kinds, all five sizes/custom
size and color, inline accessible-status composition, and a complete custom
Widget/Graphic example. A custom loader is ordinary Widget composition, not a
global CSS/provider registry. Use one named loading region and decorative child
loaders to avoid repeated announcements. Runtime tests include phased drawing
identity; native `feedback_docs` checks changing real pixels, zero source writes
and reduced-motion cache stability in all four themes.
