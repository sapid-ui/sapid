use super::showcase_examples::DetailSection;
mod button_groups;
mod button_loading;
mod button_sections;
mod button_sizes;
mod button_variants;
mod close_custom;
mod close_sizes;
mod close_states;
mod copy_feedback;
mod copy_reset;
mod copy_source;
mod copy_timer;
mod file_custom;
mod file_multiple;
mod file_single;
mod icon_group;
mod icon_loading;
mod icon_sizes;
mod icon_variants;
mod toggle_controlled;
mod toggle_groups;
mod toggle_states;
mod unstyled_custom;
mod unstyled_link;
mod unstyled_states;
pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match id {
        "action-icon" => vec![
            DetailSection {
                id: "icon-sizes",
                title: "Sizes and square hit areas",
                note: "Five square targets are 24, 30, 36, 42 and 50 CSS pixels. Icon drawing size is independent. Each action has an explicit accessible name; decorative SVG content does not add a focus stop.",
                example: Box::new(icon_sizes::demo(theme.clone())),
                source: include_str!("icon_sizes.rs"),
            },
            DetailSection {
                id: "icon-variants",
                title: "Variants and contrast",
                note: "Default, Filled, Light, Outline and Subtle use theme colors. The caller chooses an icon color suitable for the background. Explicit style declarations override these finite appearances.",
                example: Box::new(icon_variants::demo(theme.clone())),
                source: include_str!("icon_variants.rs"),
            },
            DetailSection {
                id: "icon-loading",
                title: "Loading, disabled and retained icons",
                note: "loading displays a decorative rotating indicator, sets busy and disables activation. The original caller icon is suspended and resumed with the same identity. Current enabled/loading sources are checked before both typed callbacks and generic observers.",
                example: Box::new(icon_loading::demo(theme.clone())),
                source: include_str!("icon_loading.rs"),
            },
            DetailSection {
                id: "icon-group",
                title: "Adjacent named actions",
                note: "Use a layout Group for independent named icon actions. It does not silently add toolbar arrow navigation. ButtonGroup.toolbar provides explicit roving navigation for text buttons.",
                example: Box::new(icon_group::demo(theme.clone())),
                source: include_str!("icon_group.rs"),
            },
        ],
        "button" => vec![
            DetailSection {
                id: "button-variants",
                title: "Button variants",
                note: "The six finite variants share native button activation and disabled semantics. Danger uses the danger palette; label remains the accessible name.",
                example: Box::new(button_variants::demo(theme.clone())),
                source: include_str!("button_variants.rs"),
            },
            DetailSection {
                id: "button-sizes",
                title: "Sizes and compact controls",
                note: "Five size tokens affect font, minimum height and padding. compact reduces vertical padding and minimum height. Explicit minimum/height overrides remain the application responsibility.",
                example: Box::new(button_sizes::demo(theme.clone())),
                source: include_str!("button_sizes.rs"),
            },
            DetailSection {
                id: "button-sections",
                title: "Left and right sections, full width",
                note: "Sections are passive inline content inside the real button. The accessible name is the label, excluding adornments. Nested interactive content is rejected. Percent width and normal whitespace make long labels fit narrow containers.",
                example: Box::new(button_sections::demo(theme.clone())),
                source: include_str!("button_sections.rs"),
            },
            DetailSection {
                id: "button-loading",
                title: "Loading and activation",
                note: "Loading keeps the label, shows a decorative indicator, exposes busy and prevents native/typed activation. This example uses a separate checkbox to simulate completion; no network request or implicit timer is started.",
                example: Box::new(button_loading::demo(theme.clone())),
                source: include_str!("button_loading.rs"),
            },
            DetailSection {
                id: "button-groups",
                title: "Joined groups and keyboard toolbars",
                note: "ButtonGroup shares theme/size and joins edges. toolbar(true) explicitly adds one roving tab stop and arrow/Home/End navigation, skipping disabled actions. A plain joined group keeps independent tab stops.",
                example: Box::new(button_groups::demo(theme.clone())),
                source: include_str!("button_groups.rs"),
            },
        ],
        "close-button" => vec![
            DetailSection {
                id: "close-sizes",
                title: "Sizes and vector glyphs",
                note: "The default cross is a centered vector Graphic, independently sized to 12, 14, 16, 18 and 20 pixels inside the five square targets. Name the actual dismiss action.",
                example: Box::new(close_sizes::demo(theme.clone())),
                source: include_str!("close_sizes.rs"),
            },
            DetailSection {
                id: "close-states",
                title: "Dismissal, disabled and restoration",
                note: "CloseButton only requests an action. Applications own dismissal, lifecycle and focus restoration. A disabled action cannot invoke typed or generic activation handlers, including a source change in the same event packet.",
                example: Box::new(close_states::demo(theme.clone())),
                source: include_str!("close_states.rs"),
            },
            DetailSection {
                id: "close-custom",
                title: "Variants and custom icons",
                note: "CloseButton shares ActionIcon appearances and permits replacement passive icon content. It retains a genuine named button rather than relying on the visual cross as its accessible name.",
                example: Box::new(close_custom::demo(theme.clone())),
                source: include_str!("close_custom.rs"),
            },
        ],
        "copy-button" => vec![
            DetailSection {
                id: "copy-timer",
                title: "Timed feedback with an explicit browser timer",
                note: "This complete source installs a cancellable 1.5 second browser timer through feedback_delay. It needs wasm-bindgen and web-sys Window timer features. SSR starts no timer; native hosts supply their own local Future. Owner cleanup cancels an outstanding timer.",
                example: Box::new(copy_timer::demo(theme.clone())),
                source: include_str!("copy_timer.rs"),
            },
            DetailSection {
                id: "copy-feedback",
                title: "Actual clipboard results",
                note: "Idle, Pending, Written, Denied, Unavailable and Failed reflect host results. Success text appears only after Written. Native hosts need an explicit clipboard service; browser permissions and secure context can reject a request.",
                example: Box::new(copy_feedback::demo(theme.clone())),
                source: include_str!("copy_feedback.rs"),
            },
            DetailSection {
                id: "copy-source",
                title: "Reactive source and disabled actions",
                note: "The current source is bounded to 65536 UTF-8 bytes before copying. Source replacement resets feedback and invalidates stale completion. Disabled prevents a new request; state_ref is output, not a command.",
                example: Box::new(copy_source::demo(theme.clone())),
                source: include_str!("copy_source.rs"),
            },
            DetailSection {
                id: "copy-reset",
                title: "Feedback lifetime and reset",
                note: "Without feedback_delay terminal feedback remains until a new request or source replacement, demonstrated here. For timed reset supply feedback_delay(|| Box::pin(your_host_delay())); one Owner-owned task is cancelled on replacement and cleanup. There is no built-in global timer or implicit service grant.",
                example: Box::new(copy_reset::demo(theme.clone())),
                source: include_str!("copy_reset.rs"),
            },
        ],
        "file-button" => vec![
            DetailSection {
                id: "file-single",
                title: "Single file and accepted types",
                note: "A real picker yields opaque host references and metadata. Accepted MIME/extension filters are hints plus bounded host validation, not content authentication. The button does not expose a filesystem path or prepopulate a file value.",
                example: Box::new(file_single::demo(theme.clone())),
                source: include_str!("file_single.rs"),
            },
            DetailSection {
                id: "file-multiple",
                title: "Multiple selection, cancellation and reset",
                note: "multiple and max_files are independent. Cancel retains accepted files; controller.clear revokes references and clears the underlying picker so selecting the same file again remains possible. Programmatic clearing is silent.",
                example: Box::new(file_multiple::demo(theme.clone())),
                source: include_str!("file_multiple.rs"),
            },
            DetailSection {
                id: "file-custom",
                title: "Custom presentation and admission limits",
                note: "Style changes the same real picker button. max_file_bytes and max_total_bytes bound admission. Selecting a file does not read or preview its contents: those require explicit controller operations and an installed host file service. Arbitrary polymorphic render-prop triggers are not part of this API.",
                example: Box::new(file_custom::demo(theme.clone())),
                source: include_str!("file_custom.rs"),
            },
        ],
        "toggle" => vec![
            DetailSection {
                id: "toggle-controlled",
                title: "Controlled pressed proposals",
                note: "pressed is the accepted source. on_pressed_change proposes a replacement, and the application can refuse it. The visible state and aria-pressed remain accepted until the source changes.",
                example: Box::new(toggle_controlled::demo(theme.clone())),
                source: include_str!("toggle_controlled.rs"),
            },
            DetailSection {
                id: "toggle-groups",
                title: "Single and multiple groups",
                note: "Stable item keys identify accepted values. Single permits at most one pressed action; Multiple allows independent choices. Groups own roving arrow/Home/End focus and skip disabled items. The caller retains controlled-state policy.",
                example: Box::new(toggle_groups::demo(theme.clone())),
                source: include_str!("toggle_groups.rs"),
            },
            DetailSection {
                id: "toggle-states",
                title: "Pressed and disabled states",
                note: "A Toggle is a persistent pressed button, not a checkbox or a navigation link. Disabled preserves accepted pressed state while blocking activation and callbacks.",
                example: Box::new(toggle_states::demo(theme.clone())),
                source: include_str!("toggle_states.rs"),
            },
        ],
        "unstyled-button" => vec![
            DetailSection {
                id: "unstyled-custom",
                title: "Custom presentation on a native button",
                note: "unstyled removes Sapid presentation. Explicit styles and reactive label build a custom action while retaining native button keyboard activation and semantics. Only passive inline content is accepted.",
                example: Box::new(unstyled_custom::demo(theme.clone())),
                source: include_str!("unstyled_custom.rs"),
            },
            DetailSection {
                id: "unstyled-states",
                title: "Disabled and focus behavior",
                note: "Removing presentation does not remove disabled protection or native keyboard focus. Applications should provide a clear focus indicator when replacing the default appearance.",
                example: Box::new(unstyled_states::demo(theme.clone())),
                source: include_str!("unstyled_states.rs"),
            },
            DetailSection {
                id: "unstyled-link",
                title: "Actions and navigation links",
                note: "Use an unstyled Button for an action and Anchor for a URL. Sapid keeps these semantic contracts explicit and does not implement an arbitrary polymorphic component prop.",
                example: Box::new(unstyled_link::demo(theme.clone())),
                source: include_str!("unstyled_link.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "action-icon" => vec![
            ("Sizes and square hit areas", "#icon-sizes"),
            ("Variants and contrast", "#icon-variants"),
            ("Loading, disabled and retained icons", "#icon-loading"),
            ("Adjacent named actions", "#icon-group"),
        ],
        "button" => vec![
            ("Button variants", "#button-variants"),
            ("Sizes and compact controls", "#button-sizes"),
            ("Left and right sections, full width", "#button-sections"),
            ("Loading and activation", "#button-loading"),
            ("Joined groups and keyboard toolbars", "#button-groups"),
        ],
        "close-button" => vec![
            ("Sizes and vector glyphs", "#close-sizes"),
            ("Dismissal, disabled and restoration", "#close-states"),
            ("Variants and custom icons", "#close-custom"),
        ],
        "copy-button" => vec![
            ("Timed feedback", "#copy-timer"),
            ("Actual clipboard results", "#copy-feedback"),
            ("Reactive source and disabled actions", "#copy-source"),
            ("Feedback lifetime and reset", "#copy-reset"),
        ],
        "file-button" => vec![
            ("Single file and accepted types", "#file-single"),
            (
                "Multiple selection, cancellation and reset",
                "#file-multiple",
            ),
            ("Custom presentation and admission limits", "#file-custom"),
        ],
        "toggle" => vec![
            ("Controlled pressed proposals", "#toggle-controlled"),
            ("Single and multiple groups", "#toggle-groups"),
            ("Pressed and disabled states", "#toggle-states"),
        ],
        "unstyled-button" => vec![
            ("Custom presentation on a native button", "#unstyled-custom"),
            ("Disabled and focus behavior", "#unstyled-states"),
            ("Actions and navigation links", "#unstyled-link"),
        ],
        _ => Vec::new(),
    }
}
