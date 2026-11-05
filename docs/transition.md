# Transition

`transition().mounted(source).content(factory)` presents content when its read-only
`Cage`/`Bond`/`Vary<bool>` is true. The caller owns that source. The wrapper is an
independent View, with no semantic role of its own. Give content its appropriate
name, role and focus policy.

`TransitionEffect` provides None, Fade (default), Scale, ScaleX, ScaleY, SlideUp,
SlideDown, SlideLeft and SlideRight, directional Fade variants, SkewUp/Down,
RotateLeft/Right and Pop (including corner origins). `TransitionTiming` provides Linear, Ease
(default), EaseIn, EaseOut and EaseInOut. Default duration is 200ms. `duration`,
`exit_duration`, `enter_delay`, `exit_delay`, `effect`, `timing` and `animated`
accept reactive sources. Both directions validate before a change is published:
duration/delay are 0–2000ms and `distance` is 0–200px (default 16). Rejected input
retains the previous accepted presentation and emits no lifecycle callback.

`on_enter` / `on_exit` observe each newly accepted configuration. Matching
`on_entered` / `on_exited` run once after completion. Reversal invalidates the old
revision. No per-frame Cage or application callback drives animation. Lifecycle
callbacks run before generic PresenceComplete observers; retiring the Owner
prevents later callbacks. A callback can change the source; current revision and
source are checked before accepting a terminal observation.

`ContentPolicy::Unmount` is the default. Content remains owned during exit and is
disposed only after matching completion. `KeepMounted` preserves the same editor,
selection and local state. Closing immediately makes the subtree hidden/inert;
retained pixels are not interactive. Transition does not move focus to an
application trigger: the application owns that choice. Put visual layout,
padding/border and custom styles on the child. The shell owns opacity, transform
and display; conflicting typed display/opacity declarations are rejected.

Initial open content settles without callbacks. `appear(true)` requests an initial
entry and its callbacks. `animated(false)`, zero duration and reduced motion settle
immediately. The browser uses shared host CSS and one revisioned completion
observer per live declaration. Unavailable ancestors, suspension and disposal
terminate or cancel current presentation; completed revisions do not replay on
resume. Unsupported browser transition primitives fall back to static settlement.

SSR/headless hosts render the accepted static state and complete subsequent
lifecycle synchronously. Hydration resets only the validated counter for the new
identity lifetime and adopts existing content. Stream patches serialize the same
presence declarations; pre-WASM CSS presentation does not provide server-side
exit-owner lifecycle callbacks. Apace supports clock-driven opacity for Fade/None and custom opacity;
animated transform effects are rejected before component changes. Wry rejects
presence. Engine pixels and protocol tests do not establish OS reader acceptance.

`TransitionEffect::Custom(TransitionStyles { .. })` defines the closed state. Open
uses opacity 1 and an identity transform. Opacity is 0–1000 thousandths, X/Y scale
0–2000 thousandths, X/Y translation ±200 CSS px, rotation ±180 degrees, X/Y skew
±45 degrees, and X/Y origin 0–100 percent. Common presentation belongs on the
content. Custom opacity works on Apace; animated transforms require the browser.
All ten fields validate independently before publication. Directional Fade variants
combine opacity and translation; Slide variants preserve opacity while moving.
