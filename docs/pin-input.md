# PinInput / InputOTP

`pin_input()` builds one actual single-line Input with a bounded String value.
`input_otp()` configures that same editor with an explicit one-time-code autofill
hint. Native text, selection, caret, paste, deletion and password projection stay
on that editor. At most 16 passive cells draw the font-aligned segment boundaries;
they contain no copies of the code, interactive roles, focus stops or handlers.

The cells follow actual editor focus and display the theme's focus border. The
opt-in Sapid stylesheet removes the editor's extra outer focus outline in styled
mode; it leaves the real native caret and selection intact. Unstyled mode keeps
the host's ordinary outline.

```rust
use sapid::prelude::*;
use glory::{Scope, Widget};

fn verification(scope: &Scope) -> impl Widget + use<> {
    let code = scope.cage(String::new());
    input_otp()
        .length(6)
        .name("Verification code")
        .value(code)
        .on_value_change(move |proposal| code.revise(|value| *value = proposal))
        .on_complete(move |_| {
            // Application policy decides whether and how to verify the code.
        })
}
```

`length` is static, defaults to 6 and accepts 1–16. Empty and partial strings are
valid. Digits accepts ASCII `0`–`9`; `PinAlphabet::AsciiAlphanumeric` additionally
accepts ASCII letters and preserves case. Leading zeros remain data. Whitespace,
separators and full-width digits are rejected as a whole; there is no implicit
stripping, Unicode transliteration, regex engine, uppercasing or numeric parse.
Code length and alphabet are checked by borrowing before cloning a source.

`on_value_change` proposes the complete String. The controlled source remains
read-only to the component; `default_value` owns local state only when no source
was supplied. Rejected controlled edits are restored after the complete host
tick. `on_commit` receives the complete host value, without inventing an extra
change callback. Single-line Enter still follows the shared Input Commit then
Form Submit order. There is no automatic submit, verification, persistence,
network access or code logging.

`on_complete` runs only after a user edit/commit changes the accepted source to a
full value. Controlled refusal, repeated equal events, partial values and silent
program changes cannot complete. A different accepted edit at full length may
complete again. Internal behavior and typed callbacks run before generic event
observers, while the registering Owner is live.

Invalid or oversized host proposals call `on_reject(PinRejection)` and restore
the last accepted value without faulting the App. Error variants and messages
contain policy information only. No `maxlength` truncates a pasted suffix before
the component can reject the whole proposal. Oversized generic edit events are
not cloned and delivered to observers. Program changes outside the finite domain
report a typed configuration error and retain accepted UI until the source is
corrected; the component does not rewrite that business source.

Composition preedit remains physical and unpublished. One current preedit of at
most 128 UTF-8 bytes is retained; the terminal final value is validated against
the code policy. A finite current-revision/value echo guard prevents duplicate
terminal publication. Missed composition-start is recovered from composing input
events. No value/revision history or polling is retained. This exercises host IME
protocol; it does not establish real OS IME acceptance.

## Physical layout and fields

The default presentation uses spaced cells, with an 8px `gap` and horizontal
`cell_padding` taken from the selected control-size tokens. Both configuration
values accept 0–32 finite CSS pixels. `gap(0.)` joins adjacent segments. Each
cell is exactly one real monospace zero-glyph advance plus twice its padding;
the editor uses the same glyph advance with `letter-spacing = 2 * padding + gap`.
A bounded trailing editing area keeps the real caret visible after a complete
code. There is no measured-pixel estimate, hidden editor, mirrored text, separate
cell value or simulated caret. Input focus changes the actual cell borders.

Common `class`/`style` apply to the outer presentation. Its inner frame retains
the configured glyph/pixel geometry when the outer width changes. A constrained
outer width provides a local horizontal viewport for wide codes. Font size and
line height can be customized, including reactively; the font family must remain
`Family::Mono`. An invalid family or a conflicting explicit `letter_spacing`
reports a configuration error and retains accepted style. Configure tracking
through `gap` and `cell_padding` so cells and the editor remain aligned.
`unstyled` retains the one editor and usable geometry while removing theme paint.

`name`, `node_ref`, `focus` and event observers target the actual Input.
`field(...).control(|field| input_otp().field(field))` connects label, description,
required, error, readonly and disabled state to it. `Field.form_name` or the
explicit `form_name` supplies the successful control name; the full String is one
successful value. Files and form transport remain outside this control.

`masked(source)` accepts a static boolean or Cage/Bond/Vary and uses real
`Purpose::Password` while true. Purpose changes patch the same editor without
changing its value revision or composition state. Secret text never enters decorative
Text nodes or accessible cell names. It retains native password caret/selection
and the existing native grapheme-to-bullet projection. Masking does not encrypt
the business String or imply secure server handling. Production applications
must avoid logging values in their own callbacks.

## Shared host contract

`Length::Ch` retains the final value bit (63) of the u64 support mask.
`Length::ChPx { ch, px }` adds a finite two-term sum and independently requires
both existing Ch and Px capabilities; it does not add a 65th value bit.
Dimensions require nonnegative finite coefficients, while insets may be signed.
`Style::letter_spacing(px)` independently checks a finite signed pixel value;
explicit zero resets inherited spacing. HTML/Record profile 13 and Apace profile
14 include this contract. Maximum sizes still use `Limit::{None,Px,Percent}`.
The optional CSS parser accepts only the finite `calc(ch + px)`/`calc(ch - px)`
form, and checks overridden original declarations before installation. Padding
and gaps retain their existing pixel-only domains. Default graphs gain no parser.

Browser and native editor layout use the same declarations. The actual native
font-layout regression checks glyph advances, mixed dimensions, password display
projection and identity across spacing changes and resets at multiple font sizes.
This is engine-level evidence; OS IME and assistive technology acceptance remain
separate from these layout/protocol checks.

## Evidence

- `tests/pin_inputs.rs`: 10 passed; single editor/cell budget, whole String,
  controlled refusal/completion and callback order, million-byte source and host
  refusal, composition final/echo/retirement, Field, password and unstyled states.
  `D:/BuildCache/sapid/pin-input-rust.log`.
- `src/pin_model.rs`: 6 Cargo tests passed. No codes in rejection formatting.
  `D:/BuildCache/sapid/pin-model-rust.log`.
- Glory Style all-features tests passed, including Ch 3: last-bit support,
  independent rejection for all eight Length properties, finite/signed domains,
  original CSS declaration checking and no accidental maximum/grid/pixel widening.
- Glory render input-hint target 1 and HTML integration 1 passed. All 14 HTML
  library tests passed, including expanded full-shape/field delta oracle cases
  for Purpose/hints/Clear and every Ch property. Shared strict Clippy passed.
- Glory native Ch 2 passed, covering 3 font sizes × plain/password, actual
  zero/bullet advances, reactive width and stable engine identity, plus actual
  preferred/minimum dimensions and signed top/left/right/bottom offsets.
  `D:/BuildCache/sapid/pin-ch-native.log`.
- Glory native driver 1 passed for plain/password: a real pointer click at 2ch
  produces UTF-16 caret 2, native Backspace/Home/Right work, whole selection
  replacement through editor/Input dispatch retains identity. This injects the
  host's post-clipboard editor operation, not a claim of OS clipboard integration.
  `D:/BuildCache/sapid/pin-ch-native-driver.log`.
- `examples/native/tests/pin_input.rs`: 2 passed. Four themes × plain/password
  have actual matching cell/editor geometry, every cell hits the actual editor,
  focus and sampled theme pixels. Large unstyled/readonly/disabled/control refusal,
  identity, reference cleanup and zero pending queries are covered.
  `D:/BuildCache/sapid/pin-input-native.log`.
- Sapid focused strict Clippy, Web WASM strict Clippy and SSR bin check passed.
  Every compiler/cache/temp location was verified on D.
- Fixed **Web23 actual 12/12 passed**, Chromium 148.0.7778.96, errors[]:
  four themes × CSR/hydration, actual clipboard and whole rejection,
  native selection/caret, dirty takeover, composition protocol, Owner, touch and
  browser AX. `D:/BuildCache/sapid/pin-input-browser-root23.json`.
  Light/dark screenshots were inspected; the documented compact contiguous
  glyph columns retain their actual native geometry.
- Password privacy uses actual CDP Accessibility tree values. Playwright's
  DOM-derived `ariaSnapshot` includes raw HTML input.value even for Password;
  the independent probe confirms the real Chromium AX value is masked.
  `D:/BuildCache/sapid/pin-password-actual-ax-probe.json` preserves only booleans,
  lengths and role. The initial wrong-oracle failure is retained separately in
  `pin-input-browser-root23-dom-snapshot-before-fix.log`.

Fixtures use documented public test vectors only; status and reports contain
lengths, counters and state booleans. No production codes are included in test
logs or screenshots. Physical touch devices, OS autofill, OS IME and screen-reader
announcements remain separate acceptance work.
