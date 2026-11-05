# Buttons and icon actions

`button()` is an intrinsic named Button. `ButtonVariant` provides Primary,
Secondary, Danger, Light, Outline and Subtle. Five sizes share control tokens;
`compact` reduces vertical space. `left_section` / `right_section` accept passive
inline content and keep the label as the accessible name. Nested interactive
children fail before mounting. Full-width long labels can use explicit percent
width and `WhiteSpace::Normal`.

`loading(Vary<bool>)` displays a passive rotating Graphic, exposes busy and
blocks activation. The original label and Button identity remain. There is no
implicit timer or operation. Current enabled/loading values are checked before
both typed and generic activation callbacks, even within the same event packet.

`action_icon(name).icon(content)` requires a nonempty name and caller content.
Its targets are exactly 24/30/36/42/50 pixels for XS–XL; the icon's display size
is independent. Default/Filled/Light/Outline/Subtle are finite typed variants.
`loading` suspends the original caller icon and resumes those same roots when
finished, without rebuilding it. The decorative indicator adds no focus stop.
Caller drawing colors should contrast with the chosen background.

`close_button()` uses a centered vector cross, 12/14/16/18/20 pixels. Give each
instance a meaningful `name`, such as "Dismiss announcement". Applications own
actual dismissal, retained content and focus restoration. `icon` replaces its
passive content; variants, loading and disabled behavior match ActionIcon.

`button_group()` joins button edges. `toolbar(true)` explicitly enables one
roving tab stop with arrows/Home/End and disabled skipping. A plain group keeps
normal button tab stops. Independent ActionIcons can use a layout Group;
layout alone never claims toolbar keyboard behavior.

`toggle()` is a persistent pressed Button. `pressed` is controlled;
`on_pressed_change` only proposes. ToggleGroup uses stable keys, Single/Multiple
selection, roving focus and disabled skipping. Disabled retains accepted pressed
state. Use Check for checkbox semantics and Anchor for URL navigation.

`button().unstyled()` removes Sapid presentation while retaining native action,
keyboard and disabled behavior. Explicit styles win; custom presentation should
retain a visible focus indicator. This API does not have an arbitrary polymorphic
root/component prop.

All seven public pages in `examples/web/src/showcase_button_docs` contain complete
compiled `demo(theme)` sources, not partial recipes. CopyButton feedback/timer
and FileButton grants/reset are documented alongside those examples and in
[files](files.md). Their host services remain explicit.

Validation uses component tests, four-theme native geometry/retained identity and
browser keyboard/click tests at desktop/mobile widths. Native CPU/model evidence
does not establish OS input, system clipboard/dialog or screen-reader acceptance.

References: [Button](https://mantine.dev/core/button/),
[ActionIcon](https://mantine.dev/core/action-icon/),
[CloseButton](https://mantine.dev/core/close-button/),
[CopyButton](https://mantine.dev/core/copy-button/),
[FileButton](https://mantine.dev/core/file-button/),
[Toggle](https://mantine.dev/core/toggle/),
[UnstyledButton](https://mantine.dev/core/unstyled-button/).
