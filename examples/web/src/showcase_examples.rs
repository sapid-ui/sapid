//! Widgets mounted directly by the Rust documentation site, one demo at a time.
use glory::prelude::*;

pub fn demo(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
    let selected = crate::showcase_catalog::find(id).expect("catalogued widget");
    Box::new(view().class("site-example").fill((selected.build)(theme)))
}

pub struct DetailSection {
    pub id: &'static str,
    pub title: &'static str,
    pub note: &'static str,
    pub example: Box<dyn Widget>,
    pub source: &'static str,
}

fn color_input_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        ColorEyeDropper, ColorFormat, ColorInputSectionPosition, button, color_input, field, group,
        stack,
    };

    let formats_theme = theme.clone();
    let formats = widget(move |scope| {
        let specifications = [
            ("hex", ColorFormat::Hex, "#228be6"),
            ("hexa", ColorFormat::Hexa, "#228be6b3"),
            ("rgb", ColorFormat::Rgb, "rgb(34, 139, 230)"),
            ("rgba", ColorFormat::Rgba, "rgba(34, 139, 230, 0.7)"),
            ("hsl", ColorFormat::Hsl, "hsl(208, 82%, 52%)"),
            ("hsla", ColorFormat::Hsla, "hsla(208, 82%, 52%, 0.7)"),
        ];
        let mut fields = Vec::new();
        for (key, format, initial) in specifications {
            let value = scope.cage(initial.to_owned());
            fields.push(
                field(format!("showcase-color-format-{key}"))
                    .theme(formats_theme.clone())
                    .label(key.to_uppercase())
                    .control(move |field| {
                        color_input(format!("showcase-color-format-{key}"))
                            .field(field)
                            .format(format)
                            .value(value)
                            .on_value_change(move |next| value.revise(|old| *old = next))
                    }),
            );
        }
        stack()
            .theme(formats_theme.clone())
            .gap(12.)
            .fill(fields)
            .show_in(scope)
    });

    let draft_theme = theme.clone();
    let draft = widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        color_input("showcase-color-invalid-draft")
            .theme(draft_theme.clone())
            .value(value)
            .fix_on_blur(false)
            .on_value_change(move |next| value.revise(|old| *old = next))
            .show_in(scope)
    });

    let commit_theme = theme.clone();
    let commit_demo = widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        let committed = scope.cage("No committed change yet".to_owned());
        stack()
            .theme(commit_theme.clone())
            .gap(8.)
            .fill((
                color_input("showcase-color-commit")
                    .theme(commit_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next))
                    .on_commit(move |next| {
                        committed.revise(|text| *text = format!("Committed: {next}"))
                    }),
                glory::ui::text().content(scope.bond(move || committed.get())),
            ))
            .show_in(scope)
    });

    let modes_theme = theme.clone();
    let _modes = widget(move |scope| {
        let swatches = ["#2f9e44", "#1971c2", "#7048e8", "#e64980", "#f08c00"];
        let picker = scope.cage("#1971c2".to_owned());
        let only = scope.cage("#7048e8".to_owned());
        stack()
            .theme(modes_theme.clone())
            .gap(14.)
            .fill((
                field("showcase-color-disallow")
                    .theme(modes_theme.clone())
                    .label("Picker only input")
                    .control(move |field| {
                        color_input("showcase-color-disallow")
                            .field(field)
                            .value(picker)
                            .disallow_input(true)
                            .swatches(swatches)
                            .swatches_per_row(5)
                            .on_value_change(move |next| picker.revise(|old| *old = next))
                    }),
                field("showcase-color-swatches-only")
                    .theme(modes_theme.clone())
                    .label("Swatches only")
                    .control(move |field| {
                        color_input("showcase-color-swatches-only")
                            .field(field)
                            .value(only)
                            .disallow_input(true)
                            .show_picker(false)
                            .swatches(swatches)
                            .swatches_per_row(5)
                            .close_on_color_swatch_click(true)
                            .on_value_change(move |next| only.revise(|old| *old = next))
                    }),
            ))
            .show_in(scope)
    });

    let states_theme = theme.clone();
    let _states = widget(move |scope| {
        let normal = scope.cage("#228be6".to_owned());
        group()
            .theme(states_theme.clone())
            .gap(14.)
            .wrap(glory::style::FlexWrap::Wrap)
            .fill((
                field("showcase-color-error")
                    .theme(states_theme.clone())
                    .label("Error")
                    .error("Choose an approved color")
                    .control(move |field| {
                        color_input("showcase-color-error")
                            .field(field)
                            .value(normal)
                            .on_value_change(move |next| normal.revise(|old| *old = next))
                    }),
                field("showcase-color-disabled")
                    .theme(states_theme.clone())
                    .label("Disabled")
                    .enabled(false)
                    .control(|field| color_input("showcase-color-disabled").field(field)),
                field("showcase-color-success")
                    .theme(states_theme.clone())
                    .label("Success")
                    .control(|field| {
                        color_input("showcase-color-success")
                            .field(field)
                            .success(true)
                    }),
                field("showcase-color-readonly")
                    .theme(states_theme.clone())
                    .label("Read only")
                    .readonly(true)
                    .control(|field| color_input("showcase-color-readonly").field(field)),
            ))
            .show_in(scope)
    });

    let sections_theme = theme.clone();
    let sections = widget(move |scope| {
        let loading = scope.cage(true);
        let first = scope.cage("#228be6".to_owned());
        let second = scope.cage("#2f9e44".to_owned());
        stack()
            .theme(sections_theme.clone())
            .gap(12.)
            .fill((
                color_input("showcase-color-loading")
                    .theme(sections_theme.clone())
                    .value(first)
                    .loading(loading)
                    .loading_position(ColorInputSectionPosition::Right)
                    .on_value_change(move |next| first.revise(|old| *old = next)),
                button()
                    .theme(sections_theme.clone())
                    .label(scope.bond(move || {
                        if loading.get() {
                            "Stop loading".to_owned()
                        } else {
                            "Start loading".to_owned()
                        }
                    }))
                    .on_press(move || loading.revise(|value| *value = !*value)),
                color_input("showcase-color-sections")
                    .theme(sections_theme.clone())
                    .value(second)
                    .left_section(glory::ui::text().content("L"))
                    .right_section(glory::ui::text().content("R"))
                    .on_value_change(move |next| second.revise(|old| *old = next)),
            ))
            .show_in(scope)
    });

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                color_input("showcase-color-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text()
                    .class("showcase-color-controlled-value")
                    .content(scope.bond(move || format!("Value: {}", value.get()))),
            ))
            .show_in(scope)
    });

    let loading_theme = theme.clone();
    let loading = widget(move |scope| {
        let pending = scope.cage(true);
        stack()
            .theme(loading_theme.clone())
            .gap(8.)
            .fill((
                color_input("showcase-color-loading-only")
                    .theme(loading_theme.clone())
                    .loading(pending),
                button()
                    .theme(loading_theme.clone())
                    .label(scope.bond(move || {
                        if pending.get() {
                            "Stop loading".to_owned()
                        } else {
                            "Start loading".to_owned()
                        }
                    }))
                    .on_press(move || pending.revise(|value| *value = !*value)),
            ))
            .show_in(scope)
    });

    let free_theme = theme.clone();
    let disable_free_input = color_input("showcase-color-disable-free-input")
        .theme(free_theme)
        .disallow_input(true);

    let swatches_theme = theme.clone();
    let swatches = color_input("showcase-color-with-swatches")
        .theme(swatches_theme)
        .swatches(["#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf"])
        .swatches_per_row(5);

    let close_theme = theme.clone();
    let close_on_swatch = color_input("showcase-color-close-swatch")
        .theme(close_theme)
        .default_open(true)
        .swatches(["#fa5252", "#12b886", "#228be6"])
        .close_on_color_swatch_click(true);

    let hidden_theme = theme.clone();
    let hidden_dropdown = color_input("showcase-color-hide-dropdown")
        .theme(hidden_theme)
        .show_picker(false);

    let eye_theme = theme.clone();
    let eye_dropper = widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        stack()
            .theme(eye_theme.clone())
            .gap(8.)
            .fill((
                color_input("showcase-color-eye-dropper")
                    .theme(eye_theme.clone())
                    .value(value)
                    .eye_dropper(ColorEyeDropper::new(|| async {
                        Ok(Some(glory::style::Color(18, 184, 134, 255)))
                    }))
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text()
                    .class("showcase-color-eye-dropper-value")
                    .content(scope.bond(move || format!("Selected: {}", value.get()))),
            ))
            .show_in(scope)
    });

    let eye_icon_theme = theme.clone();
    let eye_dropper_icon = widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        color_input("showcase-color-eye-dropper-icon")
            .theme(eye_icon_theme.clone())
            .value(value)
            .eye_dropper(ColorEyeDropper::new(|| async {
                Ok(Some(glory::style::Color(250, 82, 82, 255)))
            }))
            .eye_dropper_label("Use custom sampler")
            .eye_dropper_icon(glory::ui::text().content("◎"))
            .on_value_change(move |next| value.revise(|old| *old = next))
            .show_in(scope)
    });

    let state_error = field("showcase-color-state-error")
        .theme(theme.clone())
        .label("Error")
        .error("Choose an approved color")
        .control(|field| color_input("showcase-color-state-error").field(field));
    let state_success = field("showcase-color-state-success")
        .theme(theme.clone())
        .label("Success")
        .control(|field| {
            color_input("showcase-color-state-success")
                .field(field)
                .success(true)
        });
    let state_disabled = field("showcase-color-state-disabled")
        .theme(theme.clone())
        .label("Disabled")
        .enabled(false)
        .control(|field| color_input("showcase-color-state-disabled").field(field));
    let state_readonly = field("showcase-color-state-readonly")
        .theme(theme.clone())
        .label("Read only")
        .readonly(true)
        .control(|field| color_input("showcase-color-state-readonly").field(field));

    vec![
        DetailSection {
            id: "controlled",
            title: "Controlled",
            note: "The input value is owned by application state and the current formatted value is rendered beside the control.",
            example: Box::new(controlled),
            source: ".value(value)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "loading",
            title: "Loading state",
            note: "Loading is reactive, keeps the live value intact and can be toggled without rebuilding the input.",
            example: Box::new(loading),
            source: ".loading(pending)",
        },
        DetailSection {
            id: "formats",
            title: "Formats",
            note: "Hex, RGB and HSL formats support opaque and alpha variants. Alpha formats add the opacity control to the picker.",
            example: Box::new(formats),
            source: ".format(ColorFormat::Rgba)",
        },
        DetailSection {
            id: "preserve-invalid-input",
            title: "Preserve invalid input",
            note: "Disable blur correction when the application needs to keep an unfinished or invalid draft visible.",
            example: Box::new(draft),
            source: ".fix_on_blur(false)",
        },
        DetailSection {
            id: "change-end",
            title: "Committed changes",
            note: "The commit callback runs when text editing finishes or a picker interaction is committed, independently of continuous changes.",
            example: Box::new(commit_demo),
            source: ".on_commit(|value| { /* finished */ })",
        },
        DetailSection {
            id: "disable-free-input",
            title: "Disable free input",
            note: "Typing is read-only while the continuous picker remains available for deliberate color selection.",
            example: Box::new(disable_free_input),
            source: ".disallow_input(true)",
        },
        DetailSection {
            id: "with-swatches",
            title: "With swatches",
            note: "A finite palette is rendered below the continuous picker and publishes the same formatted value as direct manipulation.",
            example: Box::new(swatches),
            source: ".swatches(colors)\n.swatches_per_row(5)",
        },
        DetailSection {
            id: "close-dropdown-on-swatch-click",
            title: "Close dropdown on color swatch click",
            note: "Selecting a swatch can atomically commit the value and close the popup.",
            example: Box::new(close_on_swatch),
            source: ".close_on_color_swatch_click(true)",
        },
        DetailSection {
            id: "hide-dropdown",
            title: "Hide dropdown",
            note: "The editable color field can be used without mounting a picker popup.",
            example: Box::new(hidden_dropdown),
            source: ".show_picker(false)",
        },
        DetailSection {
            id: "eye-dropper",
            title: "Eye dropper",
            note: "The application explicitly supplies the asynchronous sampling service; the component never requests ambient browser or OS authority.",
            example: Box::new(eye_dropper),
            source: ".eye_dropper(ColorEyeDropper::new(|| async { /* host service */ }))",
        },
        DetailSection {
            id: "eye-dropper-icon",
            title: "Change eye dropper icon",
            note: "The sampler keeps an accessible label while its visible icon is supplied by the application.",
            example: Box::new(eye_dropper_icon),
            source: ".eye_dropper_label(\"Use custom sampler\")\n.eye_dropper_icon(text().content(\"◎\"))",
        },
        DetailSection {
            id: "left-right-sections",
            title: "Left and right sections",
            note: "Custom left and right sections replace the default preview area when supplied.",
            example: Box::new(sections),
            source: ".left_section(text().content(\"L\"))\n.right_section(text().content(\"R\"))",
        },
        DetailSection {
            id: "error",
            title: "Error",
            note: "Error metadata, message association and invalid state are applied to the real input.",
            example: Box::new(state_error),
            source: "field(\"color\").error(\"Choose an approved color\")",
        },
        DetailSection {
            id: "success",
            title: "Success",
            note: "Success styling is independent of the current parsed color and yields to an active error.",
            example: Box::new(state_success),
            source: ".success(true)",
        },
        DetailSection {
            id: "disabled",
            title: "Disabled",
            note: "Disabled state blocks editing, popup actions and the optional sampler service.",
            example: Box::new(state_disabled),
            source: "field(\"color\").enabled(false)",
        },
        DetailSection {
            id: "read-only",
            title: "Read only",
            note: "Read-only state preserves the value while blocking text, picker and sampler changes.",
            example: Box::new(state_readonly),
            source: "field(\"color\").readonly(true)",
        },
    ]
}

fn color_picker_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{ColorFormat, Hsva, color_picker, stack};

    let swatch_theme = theme.clone();
    let swatches = widget(move |scope| {
        let value = scope.cage(Hsva {
            hue: 208.,
            saturation: 0.82,
            value: 0.9,
            alpha: 1.,
        });
        color_picker("showcase-picker-swatches")
            .theme(swatch_theme.clone())
            .value(value)
            .swatches([
                "#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf", "#12b886", "#fab005",
            ])
            .on_value_change(move |next| value.revise(|current| *current = next))
            .show_in(scope)
    });

    let only_theme = theme.clone();
    let swatches_only = widget(move |scope| {
        let value = scope.cage(Hsva::default());
        stack()
            .theme(only_theme.clone())
            .style(glory::style::Style::new().width(glory::style::Length::Px(420.)))
            .fill((
                color_picker("showcase-picker-swatches-only")
                    .theme(only_theme.clone())
                    .value(value)
                    .show_picker(false)
                    .full_width(true)
                    .swatches([
                        "#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf", "#12b886", "#fab005",
                    ])
                    .on_value_change(move |next| value.revise(|current| *current = next)),
                glory::ui::text()
                    .class("showcase-picker-value")
                    .content(scope.bond(move || {
                        sapid::format_hex_color(
                            value.get().to_rgba().expect("valid picker value"),
                            false,
                        )
                    })),
            ))
            .show_in(scope)
    });

    let precision_theme = theme.clone();
    let precision = widget(move |scope| {
        let value = scope.cage(Hsva {
            hue: 145.,
            saturation: 0.55,
            value: 0.7,
            alpha: 0.65,
        });
        color_picker("showcase-picker-precision")
            .theme(precision_theme.clone())
            .value(value)
            .show_precision_sliders(true)
            .saturation_label("Color intensity")
            .brightness_label("Light output")
            .hue_label("Color angle")
            .alpha_label("Transparency")
            .on_value_change(move |next| value.revise(|current| *current = next))
            .show_in(scope)
    });

    let form_theme = theme.clone();
    let form_value = widget(move |scope| {
        let value = scope.cage(Hsva {
            hue: 208.,
            saturation: 0.82,
            value: 0.9,
            alpha: 0.7,
        });
        color_picker("showcase-picker-form")
            .theme(form_theme.clone())
            .value(value)
            .form_name("accent")
            .form_format(ColorFormat::Rgba)
            .on_value_change(move |next| value.revise(|current| *current = next))
            .show_in(scope)
    });

    vec![
        DetailSection {
            id: "picker-swatches",
            title: "With swatches",
            note: "Preset colors are real buttons below the continuous picker and commit the complete color atomically.",
            example: Box::new(swatches),
            source: ".swatches(colors)\n.swatches_per_row(7)",
        },
        DetailSection {
            id: "picker-swatches-only",
            title: "Swatches only and full width",
            note: "The continuous picker can be removed while the finite palette stretches to the available width.",
            example: Box::new(swatches_only),
            source: ".show_picker(false)\n.full_width(true)\n.swatches(colors)",
        },
        DetailSection {
            id: "picker-precision",
            title: "Keyboard precision and accessibility labels",
            note: "Optional saturation and brightness sliders complement the two-dimensional plane, and all four real sliders accept application-specific accessible names.",
            example: Box::new(precision),
            source: ".show_precision_sliders(true)\n.saturation_label(\"Color intensity\")\n.brightness_label(\"Light output\")\n.hue_label(\"Color angle\")\n.alpha_label(\"Transparency\")",
        },
        DetailSection {
            id: "picker-form-value",
            title: "Formatted form value",
            note: "A standalone picker retains its full HSVA model while publishing a real formatted successful-control value for form submission.",
            example: Box::new(form_value),
            source: ".form_name(\"accent\")\n.form_format(ColorFormat::Rgba)",
        },
    ]
}

fn modal_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    let centered_theme = theme.clone();
    let centered = sapid::dialog("showcase-modal-centered")
        .theme(centered_theme.clone())
        .label("Open centered modal")
        .title("Centered modal")
        .description("The shared host layer centers the panel and constrains it to the viewport.")
        .content(move || {
            sapid::text()
                .theme(centered_theme.clone())
                .content("Focus is managed by the real modal layer and returns to the trigger.")
        });

    let full_theme = theme.clone();
    let full_screen = sapid::dialog("showcase-modal-full-screen")
        .theme(full_theme.clone())
        .label("Open full-screen modal")
        .title("Full-screen workspace")
        .description("The same modal protocol fills the complete host viewport.")
        .full_screen(true)
        .content(move || {
            sapid::stack().theme(full_theme.clone()).gap(12.).fill((
                sapid::text().theme(full_theme.clone()).content(
                    "Full-screen presentation preserves Escape, outside-dismiss policy, focus trapping and return focus.",
                ),
                sapid::text().theme(full_theme.clone()).content(
                    "Long content uses the shared layer viewport scrolling contract instead of a second document.",
                ),
            ))
        });

    vec![
        DetailSection {
            id: "modal-centered",
            title: "Centered modal",
            note: "Modal dialogs are centered by the shared Layer host and keep focus, dismissal and return-focus semantics together.",
            example: Box::new(centered),
            source: "dialog(\"profile\")\n    .title(\"Centered modal\")\n    .content(...)",
        },
        DetailSection {
            id: "modal-full-screen",
            title: "Full-screen modal",
            note: "Full-screen mode fills the viewport and removes the panel radius without changing its modal behavior.",
            example: Box::new(full_screen),
            source: "dialog(\"workspace\")\n    .full_screen(true)\n    .content(...)",
        },
    ]
}

fn slider_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{SliderMark, group, slider, stack};

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage(40.0);
        stack()
            .theme(controlled_theme.clone())
            .gap(10.)
            .fill((
                slider("showcase-slider-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || format!("Value: {}", value.get()))),
            ))
            .show_in(scope)
    });

    let commit_theme = theme.clone();
    let committed = widget(move |scope| {
        let committed = scope.cage("Move the slider".to_owned());
        stack()
            .theme(commit_theme.clone())
            .gap(10.)
            .fill((
                slider("showcase-slider-commit")
                    .theme(commit_theme.clone())
                    .default_value(30.)
                    .on_value_commit(move |value| {
                        committed.revise(|text| *text = format!("Committed: {value}"))
                    }),
                glory::ui::text().content(scope.bond(move || committed.get())),
            ))
            .show_in(scope)
    });

    let bounds_theme = theme.clone();
    let bounds = group()
        .theme(bounds_theme.clone())
        .gap(18.)
        .wrap(glory::style::FlexWrap::Wrap)
        .fill((
            slider("showcase-slider-step")
                .theme(bounds_theme.clone())
                .min(-20.)
                .max(20.)
                .step(5.)
                .default_value(5.)
                .name("Minus twenty to twenty"),
            slider("showcase-slider-decimal")
                .theme(bounds_theme.clone())
                .min(0.)
                .max(1.)
                .step(0.05)
                .default_value(0.35)
                .name("Decimal value"),
        ));

    let marks_theme = theme.clone();
    let marks = stack().theme(marks_theme.clone()).gap(28.).fill((
        slider("showcase-slider-marks")
            .theme(marks_theme.clone())
            .default_value(40.)
            .marks([
                SliderMark::new(0., "0"),
                SliderMark::new(20., "20"),
                SliderMark::new(40., "40"),
                SliderMark::new(70., "70"),
                SliderMark::new(100., "100"),
            ]),
        slider("showcase-slider-restricted")
            .theme(marks_theme)
            .default_value(40.)
            .marks([
                SliderMark::new(0., "Small"),
                SliderMark::new(40., "Medium"),
                SliderMark::new(100., "Large"),
            ])
            .restrict_to_marks(true)
            .name("Restricted to marks"),
    ));

    vec![
        DetailSection {
            id: "controlled",
            title: "Controlled value",
            note: "A Cage remains the accepted source; callbacks propose changes without creating a second mirrored value.",
            example: Box::new(controlled),
            source: ".value(value)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "change-end",
            title: "Committed changes",
            note: "Use the commit callback for the final keyboard or drag value, independently from continuous updates.",
            example: Box::new(committed),
            source: ".on_value_commit(|value| { /* finished */ })",
        },
        DetailSection {
            id: "bounds-and-step",
            title: "Min, max, step and decimals",
            note: "Bounds and steps are finite values and are validated together before the live range changes.",
            example: Box::new(bounds),
            source: ".min(-20.0).max(20.0).step(5.0)",
        },
        DetailSection {
            id: "marks",
            title: "Marks and restricted selection",
            note: "Up to 64 sorted marks may carry labels. Restricted sliders snap pointer input and keyboard commands to declared marks only.",
            example: Box::new(marks),
            source: ".marks([SliderMark::new(0.0, \"Small\"), SliderMark::new(40.0, \"Medium\"), SliderMark::new(100.0, \"Large\")])\n.restrict_to_marks(true)",
        },
    ]
}

fn range_slider_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{SliderMark, range_slider, stack};

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage([25.0, 75.0]);
        stack()
            .theme(controlled_theme.clone())
            .gap(10.)
            .fill((
                range_slider("showcase-range-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    let [lower, upper] = value.get();
                    format!("Selected: {lower} – {upper}")
                })),
            ))
            .show_in(scope)
    });

    let gap_theme = theme.clone();
    let gap = stack().theme(gap_theme.clone()).gap(18.).fill((
        range_slider("showcase-range-gap")
            .theme(gap_theme.clone())
            .default_value([30., 70.])
            .minimum_gap(20.)
            .labels(["Minimum with gap".into(), "Maximum with gap".into()]),
        range_slider("showcase-range-decimal")
            .theme(gap_theme)
            .min(0.)
            .max(1.)
            .step(0.05)
            .default_value([0.25, 0.8])
            .labels(["Decimal minimum".into(), "Decimal maximum".into()]),
    ));

    let marks_theme = theme.clone();
    let marks = range_slider("showcase-range-marks")
        .theme(marks_theme)
        .default_value([20., 80.])
        .minimum_gap(20.)
        .marks([
            SliderMark::new(0., "0"),
            SliderMark::new(20., "20"),
            SliderMark::new(50., "50"),
            SliderMark::new(80., "80"),
            SliderMark::new(100., "100"),
        ])
        .restrict_to_marks(true)
        .labels(["Restricted minimum".into(), "Restricted maximum".into()]);

    let advanced_theme = theme.clone();
    let advanced = stack().theme(advanced_theme.clone()).gap(20.).fill((
        range_slider("showcase-range-maximum")
            .theme(advanced_theme.clone())
            .default_value([30., 70.])
            .minimum_gap(10.)
            .maximum_gap(40.)
            .labels(["Maximum range lower".into(), "Maximum range upper".into()]),
        range_slider("showcase-range-inverted-hidden")
            .theme(advanced_theme)
            .default_value([25., 75.])
            .inverted(true)
            .marks([
                SliderMark::new(0., "0%"),
                SliderMark::unlabeled(25.).hidden(true),
                SliderMark::new(50., "50%"),
                SliderMark::unlabeled(75.).hidden(true),
                SliderMark::new(100., "100%"),
            ])
            .restrict_to_marks(true)
            .labels(["Hidden mark lower".into(), "Hidden mark upper".into()]),
    ));

    let domain_theme = theme.clone();
    let domain = range_slider("showcase-range-domain")
        .theme(domain_theme)
        .domain([0., 100.])
        .min(10.)
        .max(90.)
        .default_value([25., 75.])
        .marks([SliderMark::new(10., "min"), SliderMark::new(90., "max")])
        .labels(["Domain minimum".into(), "Domain maximum".into()]);

    let labels_theme = theme.clone();
    let labels = stack().theme(labels_theme.clone()).gap(22.).fill((
        range_slider("showcase-range-labels")
            .theme(labels_theme.clone())
            .default_value([20., 60.])
            .value_text([Some("20 °C".into()), Some("60 °C".into())])
            .value_labels_always_on(true)
            .labels(["Temperature from".into(), "Temperature to".into()]),
        range_slider("showcase-range-no-labels")
            .theme(labels_theme)
            .default_value([30., 70.])
            .show_value_labels(false)
            .labels(["Quiet lower".into(), "Quiet upper".into()]),
        range_slider("showcase-range-scaled")
            .theme(theme.clone())
            .default_value([20., 60.])
            .scale(10.)
            .value_labels_always_on(true)
            .labels(["Scaled lower".into(), "Scaled upper".into()]),
    ));

    let collision_theme = theme.clone();
    let collisions = stack().theme(collision_theme.clone()).gap(20.).fill((
        range_slider("showcase-range-push")
            .theme(collision_theme.clone())
            .default_value([25., 65.])
            .minimum_gap(20.)
            .maximum_gap(50.)
            .labels(["Push lower".into(), "Push upper".into()]),
        range_slider("showcase-range-hold")
            .theme(collision_theme)
            .default_value([25., 65.])
            .minimum_gap(20.)
            .push_on_overlap(false)
            .labels(["Hold lower".into(), "Hold upper".into()]),
    ));

    vec![
        DetailSection {
            id: "controlled",
            title: "Controlled range",
            note: "Both thumb values are proposed and accepted atomically as one pair.",
            example: Box::new(controlled),
            source: ".value(value)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "range-gap",
            title: "Bounds, decimals and minimum gap",
            note: "The two values retain their identities and cannot cross the configured finite gap.",
            example: Box::new(gap),
            source: ".min(0.0).max(1.0).step(0.05)\n.minimum_gap(0.2)",
        },
        DetailSection {
            id: "range-marks",
            title: "Marks and restricted range",
            note: "Both thumbs use the shared mark declaration and stay on allowed values while preserving the minimum gap.",
            example: Box::new(marks),
            source: ".marks(marks)\n.restrict_to_marks(true)\n.minimum_gap(20.0)",
        },
        DetailSection {
            id: "range-advanced",
            title: "Maximum range, inverted track and hidden marks",
            note: "maximumGap prevents widening beyond a finite span; inverted changes track coloring only, and hidden marks still participate in restricted snapping.",
            example: Box::new(advanced),
            source: ".maximum_gap(40.0)\n.inverted(true)\n.marks([SliderMark::unlabeled(25.0).hidden(true), ...])",
        },
        DetailSection {
            id: "range-domain",
            title: "Independent visual domain",
            note: "domain controls the full visual track while min and max remain the actual selectable and accessible bounds.",
            example: Box::new(domain),
            source: ".domain([0.0, 100.0])\n.min(10.0)\n.max(90.0)\n.marks([SliderMark::new(10.0, \"min\"), SliderMark::new(90.0, \"max\")])",
        },
        DetailSection {
            id: "range-labels",
            title: "Control labels and accessible value text",
            note: "Value labels appear during dragging by default, can remain visible, or can be disabled. The same bounded formatted text is exposed as aria-valuetext.",
            example: Box::new(labels),
            source: ".value_text([Some(\"20 °C\".into()), Some(\"60 °C\".into())])\n.value_labels_always_on(true)\n.scale(10.0) // finite display/aria scale\n// or .show_value_labels(false)",
        },
        DetailSection {
            id: "range-collision",
            title: "Push or hold on overlap",
            note: "The default pushes the other thumb atomically when minimum or maximum range is reached. Disable pushing to keep the accepted pair unchanged instead.",
            example: Box::new(collisions),
            source: ".minimum_gap(20.0)\n.maximum_gap(50.0)\n// default: push the other thumb\n.push_on_overlap(false) // hold instead",
        },
    ]
}

fn progress_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use glory::style::Color;
    use sapid::{
        ProgressOrientation, ProgressSection, SemiCircleFillDirection, SemiCircleLabelPosition,
        SemiCircleOrientation, group, progress, ring_progress, semi_circle_progress, stack,
    };

    let geometry = group().theme(theme.clone()).gap(24.).fill((
        progress(35.)
            .theme(theme.clone())
            .width(Length::Px(220.))
            .thickness(6.),
        ring_progress(62.)
            .theme(theme.clone())
            .size(112.)
            .thickness(12.),
        semi_circle_progress(75.)
            .theme(theme.clone())
            .size(144.)
            .thickness(10.),
    ));
    let sections = [
        ProgressSection::new(20., Color(240, 62, 62, 255)),
        ProgressSection::new(30., Color(34, 139, 230, 255)),
        ProgressSection::new(10., Color(64, 192, 87, 255)),
    ];
    let compound = stack().theme(theme.clone()).gap(24.).fill((
        progress(60.)
            .theme(theme.clone())
            .name("Storage by category")
            .width(Length::Px(320.))
            .thickness(12.)
            .sections(sections),
        ring_progress(60.)
            .theme(theme.clone())
            .name("Work by category")
            .size(132.)
            .thickness(14.)
            .sections(sections),
    ));
    let states = group().theme(theme.clone()).gap(24.).fill((
        progress(40.)
            .theme(theme.clone())
            .name("Loading results")
            .width(Length::Px(220.))
            .indeterminate(true),
        ring_progress(72.)
            .theme(theme.clone())
            .name("Upload")
            .value_text("72 percent uploaded")
            .size(112.)
            .thickness(10.),
    ));
    let vertical = group().theme(theme.clone()).gap(36.).fill((
        progress(68.)
            .theme(theme.clone())
            .name("Vertical upload")
            .orientation(ProgressOrientation::Vertical)
            .length(Length::Px(180.))
            .thickness(12.),
        progress(60.)
            .theme(theme.clone())
            .name("Vertical storage by category")
            .vertical(true)
            .length(Length::Px(180.))
            .thickness(18.)
            .sections(sections),
        progress(40.)
            .theme(theme.clone())
            .name("Vertical loading")
            .vertical(true)
            .length(Length::Px(180.))
            .thickness(12.)
            .indeterminate(true),
    ));
    let semicircle_directions = group().theme(theme.clone()).gap(28.).fill((
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .label("Up · left to right")
            .filled_segment_color(Color(240, 62, 62, 255))
            .empty_segment_color(Color(64, 192, 87, 255)),
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .fill_direction(SemiCircleFillDirection::RightToLeft)
            .label("Up · right to left"),
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .semi_orientation(SemiCircleOrientation::Down)
            .label("Down · left to right"),
        semi_circle_progress(45.)
            .theme(theme.clone())
            .size(132.)
            .semi_orientation(SemiCircleOrientation::Down)
            .fill_direction(SemiCircleFillDirection::RightToLeft)
            .label("Down · right to left"),
    ));
    let semicircle_labels = group().theme(theme.clone()).gap(36.).fill((
        semi_circle_progress(30.)
            .theme(theme.clone())
            .size(180.)
            .label("Bottom"),
        semi_circle_progress(30.)
            .theme(theme.clone())
            .size(180.)
            .label("Center")
            .label_position(SemiCircleLabelPosition::Center),
    ));
    let ring_gaps = group().theme(theme.clone()).gap(28.).fill((
        ring_progress(60.)
            .theme(theme.clone())
            .size(124.)
            .thickness(12.)
            .sections(sections)
            .label("0° gap"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(124.)
            .thickness(12.)
            .sections(sections)
            .section_gap(6.)
            .label("6° gap"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(124.)
            .thickness(12.)
            .sections(sections)
            .section_gap(18.)
            .label("18° gap"),
    ));
    let ring_angles = group().theme(theme.clone()).gap(24.).fill((
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(0.)
            .root_color(Color(222, 226, 230, 255))
            .label("0°"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(90.)
            .label("90°"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(180.)
            .label("180°"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(112.)
            .sections(sections)
            .start_angle(270.)
            .label("270°"),
    ));
    let ring_caps = group().theme(theme.clone()).gap(36.).fill((
        ring_progress(60.)
            .theme(theme.clone())
            .size(132.)
            .sections(sections)
            .label("Butt caps"),
        ring_progress(60.)
            .theme(theme.clone())
            .size(132.)
            .sections(sections)
            .round_caps(true)
            .label("Rounded ends"),
    ));
    vec![
        DetailSection {
            id: "progress-geometry",
            title: "Line, ring and semicircle geometry",
            note: "Each view keeps one progressbar semantic while size and thickness change only its finite presentation geometry.",
            example: Box::new(geometry),
            source: "progress(35.0).thickness(6.0)\nring_progress(62.0).size(112.0).thickness(12.0)\nsemi_circle_progress(75.0).size(144.0)",
        },
        DetailSection {
            id: "progress-sections",
            title: "Compound sections",
            note: "Line and ring indicators accept up to 16 ordered positive sections. Their sum is validated against the exposed progress value.",
            example: Box::new(compound),
            source: ".sections([\n    ProgressSection::new(20.0, red),\n    ProgressSection::new(30.0, blue),\n    ProgressSection::new(10.0, green),\n])",
        },
        DetailSection {
            id: "progress-vertical",
            title: "Vertical orientation",
            note: "Linear indicators use the same bounded progress semantic in either orientation. Determinate and compound fills grow from the bottom; indeterminate presentation stays on the same main axis.",
            example: Box::new(vertical),
            source: ".orientation(ProgressOrientation::Vertical)\n.length(Length::Px(180.0))\n// shorthand: .vertical(true)",
        },
        DetailSection {
            id: "progress-semicircle-direction",
            title: "Semicircle orientation and fill direction",
            note: "Up and down geometry is independent from left-to-right and right-to-left value advancement. Filled and empty segments accept separate finite colors.",
            example: Box::new(semicircle_directions),
            source: ".semi_orientation(SemiCircleOrientation::Down)\n.fill_direction(SemiCircleFillDirection::RightToLeft)\n.filled_segment_color(red)\n.empty_segment_color(green)",
        },
        DetailSection {
            id: "progress-semicircle-labels",
            title: "Semicircle label position",
            note: "A bounded text label can sit at the arc baseline or in the center while the root retains its single progressbar semantic.",
            example: Box::new(semicircle_labels),
            source: ".label(\"Center\")\n.label_position(SemiCircleLabelPosition::Center)",
        },
        DetailSection {
            id: "progress-ring-gaps",
            title: "Ring section gaps",
            note: "A finite degree gap reduces each visible section without changing the ordered progress total or section offsets.",
            example: Box::new(ring_gaps),
            source: ".sections(sections)\n.section_gap(18.0)",
        },
        DetailSection {
            id: "progress-ring-angles",
            title: "Ring start angle and background",
            note: "Start angles follow the same degree convention as the reference component: 0° right, 90° bottom, 180° left and 270° top. Root color styles the unfilled track independently.",
            example: Box::new(ring_angles),
            source: ".start_angle(0.0)\n.root_color(Color(222, 226, 230, 255))",
        },
        DetailSection {
            id: "progress-ring-caps-label",
            title: "Ring caps and center label",
            note: "Rounded caps apply only to the visible outer section ends; the center label remains passive content inside the single progressbar.",
            example: Box::new(ring_caps),
            source: ".round_caps(true)\n.label(\"Rounded ends\")",
        },
        DetailSection {
            id: "progress-states",
            title: "Indeterminate and accessible values",
            note: "Unknown progress omits the current value; determinate indicators can expose bounded value text without becoming interactive.",
            example: Box::new(states),
            source: ".indeterminate(true)\n.value_text(\"72 percent uploaded\")",
        },
    ]
}

fn autocomplete_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        AutocompleteLoadingPosition, autocomplete, autocomplete_option, field, select_group, stack,
    };
    let options = || {
        ["React", "Angular", "Vue", "Svelte", "Solid", "Qwik"]
            .into_iter()
            .map(|label| autocomplete_option(label.to_lowercase(), label))
            .collect::<Vec<_>>()
    };
    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage("TypeScript".to_owned());
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                field("showcase-autocomplete-controlled")
                    .theme(controlled_theme.clone())
                    .label("Favorite library")
                    .description("Suggestions are optional; arbitrary text remains valid.")
                    .control(move |field| {
                        autocomplete("showcase-autocomplete-controlled")
                            .field(field)
                            .value(value)
                            .options(options())
                            .placeholder("Pick or enter anything")
                            .on_value_change(move |next| value.revise(|old| *old = next))
                    }),
                glory::ui::text().content(scope.bond(move || format!("Value: {}", value.get()))),
            ))
            .show_in(scope)
    });
    let first_theme = theme.clone();
    let loading = stack().theme(theme.clone()).gap(12.).fill((
        field("showcase-autocomplete-loading-right")
            .theme(theme.clone())
            .label("Loading suggestions")
            .control(move |field| {
                autocomplete("showcase-autocomplete-loading-right")
                    .field(field)
                    .options(options())
                    .loading(true)
                    .placeholder("Loading on the right")
            }),
        field("showcase-autocomplete-loading-left")
            .theme(theme.clone())
            .label("Loading on the left")
            .control(move |field| {
                autocomplete("showcase-autocomplete-loading-left")
                    .field(field)
                    .options(options())
                    .loading(true)
                    .loading_position(AutocompleteLoadingPosition::Left)
                    .placeholder("Loading on the left")
            }),
    ));
    let first = widget(move |scope| {
        let value = scope.cage(String::new());
        field("showcase-autocomplete-first")
            .theme(first_theme.clone())
            .label("Select first match on Enter")
            .control(move |field| {
                autocomplete("showcase-autocomplete-first")
                    .field(field)
                    .value(value)
                    .options(options())
                    .select_first_option_on_change(true)
                    .placeholder("Type re, then press Enter")
                    .on_value_change(move |next| value.revise(|old| *old = next))
            })
            .show_in(scope)
    });
    let blur_theme = theme.clone();
    let blur = widget(move |scope| {
        let value = scope.cage(String::new());
        field("showcase-autocomplete-blur")
            .theme(blur_theme.clone())
            .label("Select highlighted option on blur")
            .description("Use arrow keys to highlight a suggestion, then move focus away.")
            .control(move |field| {
                autocomplete("showcase-autocomplete-blur")
                    .field(field)
                    .value(value)
                    .options(options())
                    .auto_select_on_blur(true)
                    .placeholder("Highlight and leave the input")
                    .on_value_change(move |next| value.revise(|old| *old = next))
            })
            .show_in(scope)
    });
    let data_theme = theme.clone();
    let data = widget(move |scope| {
        let value = scope.cage("React".to_owned());
        field("showcase-autocomplete-data")
            .theme(data_theme.clone())
            .label("Grouped and disabled suggestions")
            .control(move |field| {
                autocomplete("showcase-autocomplete-data")
                    .field(field)
                    .value(value)
                    .group(select_group("Frontend").options([
                        autocomplete_option("react", "React"),
                        autocomplete_option("vue", "Vue"),
                    ]))
                    .group(select_group("Backend").options([
                        autocomplete_option("node", "Node"),
                        autocomplete_option("django", "Django").disabled(true),
                    ]))
                    .clearable(true)
                    .clear_label("Clear autocomplete")
                    .on_value_change(move |next| value.revise(|old| *old = next))
            })
            .show_in(scope)
    });
    let limit_theme = theme.clone();
    let limited = widget(move |scope| {
        let value = scope.cage(String::new());
        field("showcase-autocomplete-limit")
            .theme(limit_theme.clone())
            .label("Limited results")
            .description("The declaration has 24 options, but at most 5 matches are visible.")
            .control(move |field| {
                autocomplete("showcase-autocomplete-limit")
                    .field(field)
                    .value(value)
                    .options((0..24).map(|index| {
                        autocomplete_option(format!("option-{index}"), format!("Option {index}"))
                    }))
                    .limit(5)
                    .placeholder("Search 24 options")
                    .on_value_change(move |next| value.revise(|old| *old = next))
            })
            .show_in(scope)
    });
    vec![
        DetailSection {
            id: "autocomplete-controlled",
            title: "Free text and controlled value",
            note: "Autocomplete suggests values without turning the input into a constrained select.",
            example: Box::new(controlled),
            source: ".value(value)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "autocomplete-loading",
            title: "Loading suggestions",
            note: "A reactive loading state exposes busy metadata and a passive loader on either side without disabling free-text editing.",
            example: Box::new(loading),
            source: ".loading(true)\n.loading_position(AutocompleteLoadingPosition::Left)",
        },
        DetailSection {
            id: "autocomplete-first",
            title: "Select first option on change",
            note: "Every edit can highlight the first visible match so Enter commits it immediately.",
            example: Box::new(first),
            source: ".select_first_option_on_change(true)",
        },
        DetailSection {
            id: "autocomplete-blur",
            title: "Automatic selection on blur",
            note: "The highlighted enabled suggestion is committed before the dropdown closes when focus leaves.",
            example: Box::new(blur),
            source: ".auto_select_on_blur(true)",
        },
        DetailSection {
            id: "autocomplete-data",
            title: "Groups, disabled options and clear",
            note: "Groups retain order, disabled options are skipped, and the clear control restores empty free text.",
            example: Box::new(data),
            source: ".group(frontend)\n.group(backend)\n.clearable(true)",
        },
        DetailSection {
            id: "autocomplete-limit",
            title: "Bounded visible results",
            note: "limit caps visible matches without changing option identities or the free-text value.",
            example: Box::new(limited),
            source: ".options(many)\n.limit(5)",
        },
    ]
}

fn select_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        ComboboxFilter, SelectCheckPosition, SelectOptionContent, combobox, field, select,
        select_group, select_option,
    };

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage(Some("rust".to_owned()));
        glory::ui::view()
            .style(
                glory::style::Style::new()
                    .display(glory::style::Display::Flex)
                    .flex_direction(glory::style::FlexDirection::Column)
                    .gap(8.0),
            )
            .fill((
                field("showcase-select-controlled")
                    .theme(controlled_theme.clone())
                    .label("Controlled selection")
                    .control(move |field| {
                        select("showcase-select-controlled")
                            .field(field)
                            .value(value)
                            .clearable(true)
                            .options([
                                select_option("rust", "Rust"),
                                select_option("go", "Go"),
                                select_option("zig", "Zig"),
                            ])
                            .on_value_change(move |next| value.revise(|old| *old = next))
                    }),
                glory::ui::text()
                    .class("showcase-select-controlled-value")
                    .content(scope.bond(move || {
                        format!("Selected: {}", value.get().unwrap_or_else(|| "none".into()))
                    })),
            ))
            .show_in(scope)
    });

    let loading = field("showcase-select-loading")
        .theme(theme.clone())
        .label("Loading options")
        .description("The trigger remains a real select combobox and exposes busy state.")
        .control(move |field| {
            select("showcase-select-loading")
                .field(field)
                .loading(true)
                .option(select_option("pending", "Pending option"))
        });

    let content = field("showcase-select-content")
        .theme(theme.clone())
        .label("Structured option content")
        .control(move |field| {
            select("showcase-select-content")
                .field(field)
                .default_open(true)
                .default_value(Some("paris".into()))
                .check_position(SelectCheckPosition::Right)
                .group(select_group("Europe").options([
                    select_option("paris", "Paris"),
                    select_option("berlin", "Berlin"),
                ]))
                .option_content(
                    "paris",
                    SelectOptionContent::default()
                        .leading("FR")
                        .description("Capital of France")
                        .trailing("UTC+1"),
                )
                .option_content(
                    "berlin",
                    SelectOptionContent::default()
                        .leading("DE")
                        .description("Capital of Germany")
                        .trailing("UTC+1"),
                )
        });

    let geometry = field("showcase-select-geometry")
        .theme(theme.clone())
        .label("Bounded dropdown geometry")
        .control(move |field| {
            select("showcase-select-geometry")
                .field(field)
                .default_open(true)
                .max_dropdown_height(180.0)
                .popup_placement(glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Top,
                    glory::render::AnchorAlign::End,
                ))
                .options((1..=12).map(|index| {
                    select_option(format!("option-{index}"), format!("Option {index}"))
                }))
        });

    let searchable = field("showcase-select-searchable")
        .theme(theme.clone())
        .label("Searchable composition")
        .description(
            "Editable search is the explicit Combobox composition used for advanced selection.",
        )
        .control(move |field| {
            combobox("showcase-select-searchable")
                .field(field)
                .default_search("a")
                .default_open(true)
                .filter(ComboboxFilter::Prefix)
                .limit(2)
                .loading(true)
                .empty_message("Nothing found")
                .options([
                    select_option("angular", "Angular"),
                    select_option("astro", "Astro"),
                    select_option("alpine", "Alpine"),
                    select_option("ark", "Ark UI"),
                    select_option("react", "React"),
                ])
        });

    vec![
        DetailSection {
            id: "select-controlled",
            title: "Controlled and clearable",
            note: "The accepted option key remains application-owned. Clear proposes None through the same controlled change path.",
            example: Box::new(controlled),
            source: ".value(value)\n.clearable(true)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "select-loading",
            title: "Loading state",
            note: "Loading adds a bounded inline indicator and real busy metadata without replacing the select trigger or disabling it.",
            example: Box::new(loading),
            source: ".loading(true)",
        },
        DetailSection {
            id: "select-content",
            title: "Groups, option content and check position",
            note: "Finite leading, description and trailing text augments each real option. The selected check may be hidden, left or right.",
            example: Box::new(content),
            source: ".group(europe)\n.option_content(\"paris\", SelectOptionContent::default().leading(\"FR\").description(\"Capital of France\").trailing(\"UTC+1\"))\n.check_position(SelectCheckPosition::Right)",
        },
        DetailSection {
            id: "select-geometry",
            title: "Dropdown height and placement",
            note: "Dropdown height is finite and scrollable. Placement uses the shared anchored layer contract instead of host-specific positioning.",
            example: Box::new(geometry),
            source: ".max_dropdown_height(180.0)\n.popup_placement(AnchorPlacement::new(AnchorSide::Top, AnchorAlign::End))",
        },
        DetailSection {
            id: "select-searchable",
            title: "Searchable selection composition",
            note: "Search, filtering, empty results, loading and visible-result limits are provided by the explicit editable Combobox composition.",
            example: Box::new(searchable),
            source: "combobox(\"searchable\")\n    .filter(ComboboxFilter::Prefix)\n    .limit(2)\n    .empty_message(\"Nothing found\")",
        },
    ]
}

fn multi_select_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{MultiSelectLoadingPosition, multi_select, select_group, select_option, stack};

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let values = scope.cage(vec!["rust".to_owned()]);
        multi_select("showcase-multi-controlled")
            .theme(controlled_theme.clone())
            .value(values)
            .max_values(2)
            .hide_picked_options(true)
            .clearable(true)
            .placeholder("Choose up to two tools")
            .options([
                select_option("rust", "Rust"),
                select_option("glory", "Glory"),
                select_option("sapid", "Sapid"),
            ])
            .on_value_change(move |next| values.revise(|old| *old = next))
            .show_in(scope)
    });

    let loading_theme = theme.clone();
    let loading = stack().theme(loading_theme.clone()).gap(12.).fill((
        multi_select("showcase-multi-loading-right")
            .theme(loading_theme.clone())
            .loading(true)
            .placeholder("Loading on the right")
            .option(select_option("one", "One")),
        multi_select("showcase-multi-loading-left")
            .theme(loading_theme)
            .loading(true)
            .loading_position(MultiSelectLoadingPosition::Left)
            .placeholder("Loading on the left")
            .option(select_option("two", "Two")),
    ));

    let data_theme = theme.clone();
    let data = multi_select("showcase-multi-data")
        .theme(data_theme)
        .clearable(true)
        .default_value(vec!["react".to_owned()])
        .limit(4)
        .group(
            select_group("Frontend")
                .option(select_option("react", "React"))
                .option(select_option("vue", "Vue"))
                .option(select_option("svelte", "Svelte")),
        )
        .group(
            select_group("Backend")
                .option(select_option("rust", "Rust"))
                .option(select_option("go", "Go"))
                .option(select_option("java", "Java")),
        );

    vec![
        DetailSection {
            id: "multi-select-controlled",
            title: "Controlled values and limits",
            note: "Selected values stay application-owned while maxValues, hidden picked options and clear remain independent policies.",
            example: Box::new(controlled),
            source: ".value(values)\n.max_values(2)\n.hide_picked_options(true)\n.clearable(true)",
        },
        DetailSection {
            id: "multi-select-loading",
            title: "Loading suggestions",
            note: "A reactive busy state keeps search editable and places a passive loader on either side.",
            example: Box::new(loading),
            source: ".loading(true)\n.loading_position(MultiSelectLoadingPosition::Left)",
        },
        DetailSection {
            id: "multi-select-data",
            title: "Groups, clear and visible limit",
            note: "Grouped declarations preserve order, clear removes accepted pills, and limit bounds mounted matches.",
            example: Box::new(data),
            source: ".group(frontend)\n.group(backend)\n.clearable(true)\n.limit(4)",
        },
    ]
}

fn tags_input_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use glory::render::TagSeparators;
    use sapid::{TagDuplicatePolicy, TagsInputLoadingPosition, stack, tags_input};

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let state = scope.cage(sapid::TagsInputState {
            values: vec!["Rust".to_owned()],
            search: String::new(),
        });
        tags_input("showcase-tags-controlled")
            .theme(controlled_theme.clone())
            .label("Technologies")
            .placeholder("Enter a tag")
            .state(state)
            .on_change(move |next| state.revise(|old| *old = next))
            .show_in(scope)
    });

    let loading_theme = theme.clone();
    let loading = stack().theme(loading_theme.clone()).gap(12.).fill((
        tags_input("showcase-tags-loading-right")
            .theme(loading_theme.clone())
            .loading(true)
            .placeholder("Loading on the right"),
        tags_input("showcase-tags-loading-left")
            .theme(loading_theme)
            .loading(true)
            .loading_position(TagsInputLoadingPosition::Left)
            .placeholder("Loading on the left"),
    ));

    let limits = tags_input("showcase-tags-limits")
        .theme(theme.clone())
        .default_value(vec!["first".to_owned(), "second".to_owned()])
        .max_tags(3)
        .clearable(true)
        .clear_label("Clear all tags")
        .placeholder("Add one more tag");

    let blur_theme = theme.clone();
    let blur = stack().theme(blur_theme.clone()).gap(12.).fill((
        tags_input("showcase-tags-blur-on")
            .theme(blur_theme.clone())
            .label("Value is accepted on blur")
            .placeholder("Type, then leave the field"),
        tags_input("showcase-tags-blur-off")
            .theme(blur_theme)
            .label("Value is not accepted on blur")
            .accept_value_on_blur(false)
            .placeholder("Only Enter commits"),
    ));

    let parsing = tags_input("showcase-tags-parsing")
        .theme(theme)
        .label("Duplicates and split characters")
        .duplicate_policy(TagDuplicatePolicy::Allow)
        .separators(TagSeparators::EnterCommaSemicolon)
        .default_value(vec!["Tag".to_owned(), "TAG".to_owned()])
        .placeholder("Comma, space and | split values");

    vec![
        DetailSection {
            id: "tags-controlled",
            title: "Controlled values and search",
            note: "Accepted tags and editor text form one atomic application-owned state.",
            example: Box::new(controlled),
            source: ".state(state)\n.on_change(move |next| state.revise(|old| *old = next))",
        },
        DetailSection {
            id: "tags-loading",
            title: "Loading state",
            note: "A reactive busy state keeps the editor available and places a passive loader on either side.",
            example: Box::new(loading),
            source: ".loading(true)\n.loading_position(TagsInputLoadingPosition::Left)",
        },
        DetailSection {
            id: "tags-limits-clear",
            title: "Maximum tags and clear",
            note: "maxTags rejects an over-budget proposal atomically; clear removes accepted tags only while editable.",
            example: Box::new(limits),
            source: ".max_tags(3)\n.clearable(true)\n.clear_label(\"Clear all tags\")",
        },
        DetailSection {
            id: "tags-blur",
            title: "Accept value on blur",
            note: "Blur commits the current canonical draft by default and can be disabled explicitly.",
            example: Box::new(blur),
            source: ".accept_value_on_blur(false)",
        },
        DetailSection {
            id: "tags-parsing",
            title: "Duplicates and split characters",
            note: "Duplicate policy and finite split characters are explicit and paste remains an atomic proposal.",
            example: Box::new(parsing),
            source: ".duplicate_policy(TagDuplicatePolicy::Allow)\n.separators(TagSeparators::EnterCommaSemicolon)",
        },
    ]
}

fn pills_input_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{PillsInput, TagsInputLoadingPosition, pills_input, stack};

    fn controlled(theme: sapid::Theme) -> impl Widget {
        widget(move |scope| {
            let state = scope.cage(sapid::TagsInputState {
                values: vec!["Design".to_owned(), "System".to_owned()],
                search: String::new(),
            });
            pills_input("showcase-pills-controlled")
                .theme(theme.clone())
                .label("Labels")
                .state(state)
                .clearable(true)
                .placeholder("Add a label")
                .on_change(move |next| state.revise(|old| *old = next))
                .show_in(scope)
        })
    }

    let loading_theme = theme.clone();
    let loading: (PillsInput, PillsInput) = (
        pills_input("showcase-pills-loading-right")
            .theme(loading_theme.clone())
            .loading(true)
            .placeholder("Loading on the right"),
        pills_input("showcase-pills-loading-left")
            .theme(loading_theme.clone())
            .loading(true)
            .loading_position(TagsInputLoadingPosition::Left)
            .placeholder("Loading on the left"),
    );

    vec![
        DetailSection {
            id: "pills-controlled",
            title: "Controlled pill values",
            note: "Pill-shaped accepted values and editor text share the same atomic controlled state.",
            example: Box::new(controlled(theme.clone())),
            source: ".state(state)\n.clearable(true)\n.on_change(move |next| state.revise(|old| *old = next))",
        },
        DetailSection {
            id: "pills-loading",
            title: "Loading state",
            note: "Busy metadata and left or right loading indicators do not disable editing.",
            example: Box::new(stack().theme(theme.clone()).gap(12.).fill(loading)),
            source: ".loading(true)\n.loading_position(TagsInputLoadingPosition::Left)",
        },
        DetailSection {
            id: "pills-policies",
            title: "Commit policies",
            note: "The finite editor supports max tags, duplicate policy, separators and optional blur acceptance without cloning tag history.",
            example: Box::new(
                pills_input("showcase-pills-policies")
                    .theme(theme)
                    .default_value(vec!["first".to_owned(), "second".to_owned()])
                    .max_tags(3)
                    .accept_value_on_blur(false)
                    .placeholder("Press Enter to commit"),
            ),
            source: ".max_tags(3)\n.accept_value_on_blur(false)",
        },
    ]
}

fn time_input_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        DateLocale, HourCycle, LocalTime, TimeBounds, TimeFormat, TimePrecision, field, group,
        stack, time_input,
    };
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    let usage = field("showcase-time-usage")
        .theme(theme.clone())
        .label("Appointment time")
        .description("Enter the local wall-clock time")
        .control({
            let theme = theme.clone();
            move |field| {
                time_input("showcase-time-usage")
                    .theme(theme.clone())
                    .field(field)
                    .placeholder("Select a time")
            }
        });
    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage(Some(at(10, 30, 0)));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                time_input("showcase-time-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    value.with(|value| match value {
                        Some(value) => format!(
                            "Accepted: {:02}:{:02}:{:02}",
                            value.hour(),
                            value.minute(),
                            value.second()
                        ),
                        None => "Accepted: empty".to_owned(),
                    })
                })),
            ))
            .show_in(scope)
    });
    let precision = group().theme(theme.clone()).gap(20.).fill((
        time_input("showcase-time-24h")
            .theme(theme.clone())
            .name("24 hour minutes")
            .default_value(Some(at(18, 45, 0))),
        time_input("showcase-time-seconds")
            .theme(theme.clone())
            .name("24 hour seconds")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 30))),
        time_input("showcase-time-12h")
            .theme(theme.clone())
            .name("12 hour seconds")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 30))),
        time_input("showcase-time-12h-zh")
            .theme(theme.clone())
            .name("Chinese 12 hour seconds")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::ZhCn,
            ))
            .default_value(Some(at(18, 45, 30))),
    ));
    let placeholders = group().theme(theme.clone()).gap(20.).fill((
        time_input("showcase-time-placeholder")
            .theme(theme.clone())
            .name("Empty time")
            .placeholder("HH:MM"),
        time_input("showcase-time-prefilled")
            .theme(theme.clone())
            .name("Prefilled time")
            .placeholder("HH:MM")
            .default_value(Some(at(9, 15, 0))),
    ));
    let states = group().theme(theme.clone()).gap(20.).fill((
        time_input("showcase-time-bounded")
            .theme(theme.clone())
            .name("Office hours")
            .bounds(TimeBounds::new(Some(at(9, 0, 0)), Some(at(17, 0, 0))).unwrap())
            .default_value(Some(at(10, 0, 0))),
        time_input("showcase-time-readonly")
            .theme(theme.clone())
            .name("Readonly time")
            .readonly(true)
            .default_value(Some(at(11, 30, 0))),
        time_input("showcase-time-disabled")
            .theme(theme.clone())
            .name("Disabled time")
            .enabled(false)
            .default_value(Some(at(13, 0, 0))),
    ));
    vec![
        DetailSection {
            id: "time-input-usage",
            title: "Usage and field metadata",
            note: "TimeInput is one real editor connected to the shared Field label and description contract.",
            example: Box::new(usage),
            source: ".field(field)\n.placeholder(\"Select a time\")",
        },
        DetailSection {
            id: "time-input-controlled",
            title: "Controlled local time",
            note: "Valid edits propose an exact LocalTime; incomplete or invalid drafts remain in the editor without fabricating a business value.",
            example: Box::new(controlled),
            source: ".value(value)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "time-input-precision",
            title: "Hour cycle and seconds",
            note: "12/24-hour cycles, minute/second precision and locale markers are explicit finite formatting rather than ambient browser locale behavior.",
            example: Box::new(precision),
            source: ".format(TimeFormat::new(HourCycle::H12, TimePrecision::Second, DateLocale::En))\n.format(TimeFormat::new(HourCycle::H12, TimePrecision::Second, DateLocale::ZhCn))",
        },
        DetailSection {
            id: "time-input-placeholder",
            title: "Placeholder and initial value",
            note: "The placeholder uses the shared host hint layout and disappears when an accepted or draft value is present without replacing the editor.",
            example: Box::new(placeholders),
            source: ".placeholder(\"HH:MM\")\n.default_value(Some(LocalTime::new(9, 15, 0)?))",
        },
        DetailSection {
            id: "time-input-states",
            title: "Bounds, readonly and disabled",
            note: "Bounds reject rather than clamp drafts. Readonly and disabled are independent native editor states.",
            example: Box::new(states),
            source: ".bounds(TimeBounds::new(Some(start), Some(end))?)\n.readonly(true)\n.enabled(false)",
        },
    ]
}

fn time_picker_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        AnchorAlign, AnchorPlacement, AnchorSide, DateLocale, HourCycle, LocalTime, TimeBounds,
        TimeDuration, TimeFormat, TimeGridSpec, TimePickerPresetGroup, TimePrecision, field, group,
        stack, time_picker,
    };
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    let usage = field("showcase-time-picker-field")
        .theme(theme.clone())
        .label("Appointment time")
        .description("Enter hours and minutes as independent spinbutton segments")
        .control({
            let theme = theme.clone();
            move |field| {
                time_picker("showcase-time-picker-usage")
                    .theme(theme.clone())
                    .field(field)
                    .default_value(Some(at(10, 30, 0)))
            }
        });
    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let value = scope.cage(Some(at(14, 45, 0)));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                time_picker("showcase-time-picker-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .clearable(true)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    value.with(|value| match value {
                        Some(value) => {
                            format!("Selected: {:02}:{:02}", value.hour(), value.minute())
                        }
                        None => "Selected: empty".to_owned(),
                    })
                })),
            ))
            .show_in(scope)
    });
    let formats = group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-seconds")
            .theme(theme.clone())
            .name("Time with seconds")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 30))),
        time_picker("showcase-time-picker-twelve")
            .theme(theme.clone())
            .name("12 hour time")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Minute,
                DateLocale::En,
            ))
            .default_value(Some(at(18, 45, 0))),
        time_picker("showcase-time-picker-placeholders")
            .theme(theme.clone())
            .name("Time placeholders")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_placeholder("hh")
            .minute_placeholder("mm")
            .second_placeholder("ss"),
    ));
    let duration = time_picker("showcase-time-picker-duration")
        .theme(theme.clone())
        .name("Elapsed duration")
        .duration()
        .precision(TimePrecision::Second)
        .clearable(true)
        .default_value(Some(TimeDuration::new(125, 30, 45).unwrap()));
    let duration_digits = group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-duration-two-digits")
            .theme(theme.clone())
            .name("Two minimum hour digits")
            .duration()
            .min_hours_digits(2)
            .default_value(Some(TimeDuration::new(7, 5, 0).unwrap())),
        time_picker("showcase-duration-four-digits")
            .theme(theme.clone())
            .name("Four minimum hour digits")
            .duration()
            .min_hours_digits(4)
            .default_value(Some(TimeDuration::new(7, 5, 0).unwrap())),
    ));
    let paste = group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-paste-default")
            .theme(theme.clone())
            .name("Paste 17:33:43")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            )),
        time_picker("showcase-time-picker-paste-custom")
            .theme(theme.clone())
            .name("Paste noon")
            .paste_split(move |value| (value.trim() == "noon").then(|| at(12, 0, 0))),
    ));
    let dropdown = group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-dropdown")
            .theme(theme.clone())
            .name("Time controls")
            .with_dropdown(true)
            .minute_step(15)
            .default_value(Some(at(10, 30, 0))),
        time_picker("showcase-time-picker-dropdown-reversed")
            .theme(theme.clone())
            .name("Reversed 12 hour controls")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .with_dropdown(true)
            .reverse_time_controls_list(true)
            .max_dropdown_content_height(180.)
            .default_value(Some(at(18, 45, 30))),
    ));
    let steps = time_picker("showcase-time-picker-steps")
        .theme(theme.clone())
        .name("Quarter hour appointment")
        .hour_step(2)
        .minute_step(15)
        .bounds(TimeBounds::new(Some(at(8, 0, 0)), Some(at(18, 0, 0))).unwrap())
        .default_value(Some(at(10, 30, 0)));
    let grid = time_picker("showcase-time-picker-grid")
        .theme(theme.clone())
        .name("Available time slots")
        .grid(TimeGridSpec::new(at(9, 0, 0), at(12, 0, 0), 1800, 4).unwrap())
        .disable_time(at(10, 30, 0))
        .default_value(Some(at(9, 30, 0)));
    let presets = time_picker("showcase-time-picker-presets")
        .theme(theme.clone())
        .name("Meeting time")
        .presets(vec![
            at(9, 0, 0),
            at(10, 30, 0),
            at(12, 0, 0),
            at(14, 30, 0),
            at(16, 0, 0),
            at(17, 30, 0),
        ])
        .default_value(Some(at(10, 30, 0)));
    let grouped_presets = time_picker("showcase-time-picker-grouped-presets")
        .theme(theme.clone())
        .name("Grouped meeting time")
        .format(TimeFormat::new(
            HourCycle::H12,
            TimePrecision::Minute,
            DateLocale::En,
        ))
        .presets(vec![
            TimePickerPresetGroup::new("Morning", [at(9, 0, 0), at(10, 30, 0)]),
            TimePickerPresetGroup::new("Afternoon", [at(13, 30, 0), at(15, 0, 0), at(17, 30, 0)]),
        ])
        .close_on_preset_select(true)
        .default_value(Some(at(13, 30, 0)));
    let popup_theme = theme.clone();
    let controlled_popup = widget(move |scope| {
        let open = scope.cage(false);
        stack()
            .theme(popup_theme.clone())
            .gap(8.)
            .fill((
                time_picker("showcase-time-picker-controlled-popup")
                    .theme(popup_theme.clone())
                    .open(open)
                    .presets(vec![at(8, 0, 0), at(12, 0, 0), at(18, 0, 0)])
                    .on_open_change(move |next| open.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    if open.get() {
                        "Preset popup: open".to_owned()
                    } else {
                        "Preset popup: closed".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    });
    let labels_theme = theme.clone();
    let labels = widget(move |scope| {
        let status = scope.cage("Focus a time segment".to_owned());
        stack()
            .theme(labels_theme.clone())
            .gap(8.)
            .fill((
                time_picker("showcase-time-picker-labels")
                    .theme(labels_theme.clone())
                    .format(TimeFormat::new(
                        HourCycle::H12,
                        TimePrecision::Second,
                        DateLocale::En,
                    ))
                    .hour_label("Hours")
                    .minute_label("Minutes")
                    .second_label("Seconds")
                    .period_label("Day period")
                    .period_labels("पूर्वाह्न", "अपराह्न")
                    .default_value(Some(at(18, 45, 30)))
                    .on_focus(move || status.revise(|text| *text = "Time picker focused".into()))
                    .on_blur(move || status.revise(|text| *text = "Time picker blurred".into())),
                glory::ui::text().content(status),
            ))
            .show_in(scope)
    });
    let popup_layout = time_picker("showcase-time-picker-popup-layout")
        .theme(theme.clone())
        .name("Top aligned presets")
        .presets(vec![at(8, 0, 0), at(10, 0, 0), at(12, 0, 0)])
        .popup_placement(AnchorPlacement {
            side: AnchorSide::Top,
            align: AnchorAlign::Start,
            gap: 8.,
            viewport_padding: 12.,
            flip: false,
            shift: true,
            ..AnchorPlacement::default()
        })
        .popup_width(320.);
    let states = group().theme(theme.clone()).gap(20.).fill((
        time_picker("showcase-time-picker-readonly")
            .theme(theme.clone())
            .name("Readonly time")
            .readonly(true)
            .clearable(true)
            .default_value(Some(at(11, 30, 0))),
        time_picker("showcase-time-picker-disabled")
            .theme(theme.clone())
            .name("Disabled time")
            .enabled(false)
            .clearable(true)
            .default_value(Some(at(13, 0, 0))),
    ));
    vec![
        DetailSection {
            id: "time-picker-usage",
            title: "Usage and field metadata",
            note: "Each segment is a real spinbutton. The group shares one field label, description and typed LocalTime value.",
            example: Box::new(usage),
            source: ".field(field)\n.default_value(Some(LocalTime::new(10, 30, 0)?))",
        },
        DetailSection {
            id: "time-picker-controlled",
            title: "Controlled and clearable",
            note: "The clear action proposes None through the same controlled protocol and disappears only after the source accepts it.",
            example: Box::new(controlled),
            source: ".value(value)\n.clearable(true)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "time-picker-format",
            title: "Seconds and 12-hour format",
            note: "Second precision, the 12-hour afternoon control and bounded per-segment placeholders are explicit finite format choices.",
            example: Box::new(formats),
            source: ".format(TimeFormat::new(HourCycle::H12, TimePrecision::Second, DateLocale::En))\n.hour_placeholder(\"hh\")\n.minute_placeholder(\"mm\")\n.second_placeholder(\"ss\")",
        },
        DetailSection {
            id: "time-picker-duration",
            title: "Duration type",
            note: "The typed duration variant permits hours beyond 24 without pretending the value is a wall-clock LocalTime. Its hour editor grows with the finite value.",
            example: Box::new(duration),
            source: ".duration()\n.precision(TimePrecision::Second)\n.default_value(Some(TimeDuration::new(125, 30, 45)?))",
        },
        DetailSection {
            id: "time-picker-duration-digits",
            title: "Minimum hour digits",
            note: "Duration placeholders and committed values honor one to six minimum hour digits while retaining the same editor identity.",
            example: Box::new(duration_digits),
            source: ".duration()\n.min_hours_digits(4)\n.default_value(Some(TimeDuration::new(7, 5, 0)?))",
        },
        DetailSection {
            id: "time-picker-paste",
            title: "Paste complete times",
            note: "Paste a complete value into any segment. The default bounded parser accepts ISO time text such as 17:33:43; applications can provide a finite custom parser such as the word noon.",
            example: Box::new(paste),
            source: ".format(TimeFormat::new(HourCycle::H24, TimePrecision::Second, DateLocale::En))\n.paste_split(|text| parse_application_time(text))",
        },
        DetailSection {
            id: "time-picker-dropdown",
            title: "Time controls dropdown",
            note: "The anchored popup can render independent hour, minute, second and day-period columns. Segment steps, reversed lists and a bounded scroll height apply to the real controls.",
            example: Box::new(dropdown),
            source: ".with_dropdown(true)\n.minute_step(15)\n.reverse_time_controls_list(true)\n.max_dropdown_content_height(180.0)",
        },
        DetailSection {
            id: "time-picker-steps",
            title: "Bounds and segment steps",
            note: "Hours, minutes and seconds have independent spin steps. Bounds reject invalid proposals instead of silently clamping them.",
            example: Box::new(steps),
            source: ".hour_step(2)\n.minute_step(15)\n.bounds(TimeBounds::new(Some(start), Some(end))?)",
        },
        DetailSection {
            id: "time-picker-grid",
            title: "Time grid and disabled slots",
            note: "The optional finite grid and typed segments share one value. Disabled slots are unavailable from either input path.",
            example: Box::new(grid),
            source: ".grid(TimeGridSpec::new(start, end, 1800, 4)?)\n.disable_time(blocked)",
        },
        DetailSection {
            id: "time-picker-presets",
            title: "Preset popup",
            note: "Focusing a segment or activating the adjacent trigger opens a real anchored popup. Presets use the same controlled value path as typed edits.",
            example: Box::new(presets),
            source: ".presets(vec![at(9, 0), at(10, 30), at(12, 0)])",
        },
        DetailSection {
            id: "time-picker-grouped-presets",
            title: "Grouped presets",
            note: "Finite labeled groups preserve the active value, disable unavailable times, and can close the popup after an accepted selection.",
            example: Box::new(grouped_presets),
            source: ".presets(vec![TimePickerPresetGroup::new(\"Morning\", values)])\n.close_on_preset_select(true)",
        },
        DetailSection {
            id: "time-picker-controlled-popup",
            title: "Controlled preset popup",
            note: "Open state is controllable and Escape or outside dismissal proposes false through the same callback without bypassing the source.",
            example: Box::new(controlled_popup),
            source: ".open(open)\n.on_open_change(move |next| open.revise(|old| *old = next))",
        },
        DetailSection {
            id: "time-picker-labels-focus",
            title: "Labels and focus boundary",
            note: "Each segment and the period toggle has an independent accessible label. The first focus entry and final exit are observable at the composite boundary.",
            example: Box::new(labels),
            source: ".hour_label(\"Hours\")\n.period_label(\"Day period\")\n.period_labels(\"पूर्वाह्न\", \"अपराह्न\")\n.on_focus(...)\n.on_blur(...)",
        },
        DetailSection {
            id: "time-picker-popup-layout",
            title: "Popup placement and width",
            note: "The presets popup uses the shared typed anchor placement model and a bounded explicit width instead of arbitrary browser positioning props.",
            example: Box::new(popup_layout),
            source: ".popup_placement(AnchorPlacement { side: AnchorSide::Top, .. })\n.popup_width(320.0)",
        },
        DetailSection {
            id: "time-picker-states",
            title: "Readonly and disabled",
            note: "Readonly and disabled state applies to every segment, period toggle, grid control and clear action.",
            example: Box::new(states),
            source: ".readonly(true)\n.enabled(false)\n.clearable(true)",
        },
    ]
}

fn date_time_picker_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        DateLocale, DatePickerLevel, DateTimeDisplayFormat, DateTimePolicy, DateTimePreset,
        DateTimeRangePreset, DateTimeRangeValue, DatesProvider, HourCycle, LocalTime, TimeFormat,
        TimePrecision, YearMonth, date_time_picker, date_time_range_picker, field, group,
        inline_date_time_picker, inline_date_time_range_picker, parse_local_date_time, stack,
    };
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    let usage = field("showcase-date-time-field")
        .theme(theme.clone())
        .label("Appointment")
        .description("Choose a civil date and time")
        .control({
            let theme = theme.clone();
            move |field| {
                date_time_picker("showcase-date-time-usage", provider, DateTimePolicy::Civil)
                    .theme(theme.clone())
                    .field(field)
                    .default_value(Some(value("2026-11-04T10:30")))
            }
        });
    let seconds = date_time_picker(
        "showcase-date-time-seconds",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Date and time with seconds")
    .format(TimeFormat::new(
        HourCycle::H12,
        TimePrecision::Second,
        DateLocale::En,
    ))
    .display_format(DateTimeDisplayFormat::DayMonthYear)
    .placeholder("Choose date and time")
    .default_value(Some(value("2026-11-04T18:45:30")));
    let presets = date_time_picker(
        "showcase-date-time-presets",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Schedule from a preset")
    .default_time(LocalTime::new(9, 30, 0).unwrap())
    .presets([
        DateTimePreset::new("Yesterday", value("2026-11-03T09:30")),
        DateTimePreset::new("Today", value("2026-11-04T09:30")),
        DateTimePreset::new("Tomorrow", value("2026-11-05T09:30")),
    ]);
    let time_options = date_time_picker(
        "showcase-date-time-time-options",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Configured time controls")
    .format(TimeFormat::new(
        HourCycle::H12,
        TimePrecision::Second,
        DateLocale::En,
    ))
    .time_dropdown(true)
    .time_steps(1, 15, 10)
    .reverse_time_controls(true)
    .max_time_dropdown_height(180.)
    .default_value(Some(value("2026-11-04T18:45:30")));
    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let current = scope.cage(Some(value("2026-11-04T14:45")));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                date_time_picker(
                    "showcase-date-time-controlled",
                    provider,
                    DateTimePolicy::Civil,
                )
                .theme(controlled_theme.clone())
                .value(current)
                .clearable(true)
                .on_value_change(move |next| current.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    current
                        .get()
                        .map(sapid::format_local_date_time)
                        .unwrap_or_else(|| "No date and time selected".into())
                })),
            ))
            .show_in(scope)
    });
    let inline = inline_date_time_picker(
        "showcase-inline-date-time-detail",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Inline date and time")
    .default_time(LocalTime::new(8, 0, 0).unwrap())
    .presets([
        DateTimePreset::new("Morning", value("2026-11-04T08:00")),
        DateTimePreset::new("Evening", value("2026-11-04T18:00")),
    ])
    .default_value(Some(value("2026-11-04T08:00")));
    let calendar_controls = stack().theme(theme.clone()).gap(20.).fill((
        inline_date_time_picker(
            "showcase-inline-date-time-levels",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Date and time level navigation")
        .default_month(YearMonth::new(2026, 11).unwrap())
        .default_level(DatePickerLevel::Decade),
        date_time_picker(
            "showcase-date-time-columns",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Date and time with two months")
        .default_month(YearMonth::new(2026, 11).unwrap())
        .number_of_columns(2)
        .columns_to_scroll(1)
        .with_week_numbers(true),
        inline_date_time_range_picker(
            "showcase-inline-date-time-range-levels",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Date and time range level navigation")
        .default_month(YearMonth::new(2026, 11).unwrap())
        .default_level(DatePickerLevel::Year),
    ));
    let conference = DateTimeRangeValue::new(
        Some(value("2026-11-04T09:00")),
        Some(value("2026-11-06T17:30")),
    )
    .unwrap();
    let range_theme = theme.clone();
    let range = widget(move |scope| {
        let current = scope.cage(DateTimeRangeValue::default());
        stack()
            .theme(range_theme.clone())
            .gap(8.)
            .fill((
                date_time_range_picker("showcase-date-time-range", provider, DateTimePolicy::Civil)
                    .theme(range_theme.clone())
                    .name("Conference period")
                    .placeholder("Pick conference dates and times")
                    .value(current)
                    .clearable(true)
                    .default_times(
                        LocalTime::new(9, 0, 0).unwrap(),
                        LocalTime::new(17, 30, 0).unwrap(),
                    )
                    .presets([DateTimeRangePreset::new("Conference", conference)])
                    .on_value_change(move |next| current.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || match current.get() {
                    DateTimeRangeValue {
                        start: Some(start),
                        end: Some(end),
                    } => format!(
                        "{} – {}",
                        sapid::format_local_date_time(start),
                        sapid::format_local_date_time(end)
                    ),
                    DateTimeRangeValue {
                        start: Some(start),
                        end: None,
                    } => format!("Start: {}", sapid::format_local_date_time(start)),
                    _ => "No range selected".into(),
                })),
            ))
            .show_in(scope)
    });
    let inline_range = inline_date_time_range_picker(
        "showcase-inline-date-time-range",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Inline date and time range")
    .default_times(
        LocalTime::new(9, 0, 0).unwrap(),
        LocalTime::new(17, 30, 0).unwrap(),
    )
    .default_value(conference);
    let states = group().theme(theme.clone()).gap(20.).fill((
        date_time_picker(
            "showcase-date-time-readonly",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Readonly appointment")
        .readonly(true)
        .clearable(true)
        .default_value(Some(value("2026-11-04T11:30"))),
        date_time_picker(
            "showcase-date-time-disabled",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Disabled appointment")
        .enabled(false)
        .default_value(Some(value("2026-11-04T13:00"))),
    ));
    vec![
        DetailSection {
            id: "date-time-picker-usage",
            title: "Usage and input popup",
            note: "The compact field opens a real dialog containing the shared calendar and segmented TimePicker while retaining one typed LocalDateTime value.",
            example: Box::new(usage),
            source: ".field(field)\n.default_value(Some(parse_local_date_time(\"2026-11-04T10:30\")?))",
        },
        DetailSection {
            id: "date-time-picker-seconds",
            title: "Seconds and 12-hour time",
            note: "The time half uses the same explicit precision, hour cycle and locale contract as TimePicker.",
            example: Box::new(seconds),
            source: ".format(TimeFormat::new(HourCycle::H12, TimePrecision::Second, DateLocale::En))\n.display_format(DateTimeDisplayFormat::DayMonthYear)\n.placeholder(\"Choose date and time\")",
        },
        DetailSection {
            id: "date-time-picker-presets",
            title: "Presets and default time",
            note: "Finite labeled presets select a complete civil value. Selecting a date without a time applies the explicit default time instead of fabricating midnight.",
            example: Box::new(presets),
            source: ".default_time(LocalTime::new(9, 30, 0)?)\n.presets([DateTimePreset::new(\"Today\", value)])",
        },
        DetailSection {
            id: "date-time-picker-time-options",
            title: "TimePicker configuration",
            note: "DateTimePicker forwards finite time controls to its real TimePicker: seconds, segment steps, the controls dropdown, reversed lists and bounded popup height.",
            example: Box::new(time_options),
            source: ".time_dropdown(true)\n.time_steps(1, 15, 10)\n.reverse_time_controls(true)\n.max_time_dropdown_height(180.0)",
        },
        DetailSection {
            id: "date-time-picker-controlled",
            title: "Controlled and clearable",
            note: "The outer clear action proposes None through the same controlled value protocol and respects readonly or disabled state.",
            example: Box::new(controlled),
            source: ".value(value)\n.clearable(true)\n.on_value_change(move |next| value.revise(|old| *old = next))",
        },
        DetailSection {
            id: "date-time-picker-inline",
            title: "Inline picker",
            note: "InlineDateTimePicker exposes the same calendar, preset and segmented-time behavior without an input popup.",
            example: Box::new(inline),
            source: "inline_date_time_picker(key, provider, DateTimePolicy::Civil)\n.default_time(time)\n.presets(presets)",
        },
        DetailSection {
            id: "date-time-picker-calendar-controls",
            title: "Calendar levels and columns",
            note: "Popup, inline and atomic range variants forward the real DatePicker month/year/decade levels, controlled month anchor, bounded consecutive columns, scroll step and week rows.",
            example: Box::new(calendar_controls),
            source: ".default_level(DatePickerLevel::Decade)\n\n.number_of_columns(2)\n.columns_to_scroll(1)\n.with_week_numbers(true)",
        },
        DetailSection {
            id: "date-time-picker-range",
            title: "Date and time range",
            note: "Range mode keeps one typed value, one range calendar and two endpoint time editors. Partial starts remain explicit and Done only commits an empty or complete ordered interval.",
            example: Box::new(range),
            source: "date_time_range_picker(key, provider, policy)\n.value(range)\n.default_times(start, end)\n.presets([DateTimeRangePreset::new(label, range)])",
        },
        DetailSection {
            id: "date-time-picker-inline-range",
            title: "Inline date and time range",
            note: "The inline form exposes the same atomic range controller directly, including independent start/end times and reversal validation.",
            example: Box::new(inline_range),
            source: "inline_date_time_range_picker(key, provider, policy)\n.default_value(range)\n.default_times(start, end)",
        },
        DetailSection {
            id: "date-time-picker-states",
            title: "Readonly and disabled",
            note: "Readonly preserves inspection and focus while blocking edits; disabled makes the trigger and all popup controls unavailable.",
            example: Box::new(states),
            source: ".readonly(true)\n.enabled(false)",
        },
    ]
}

fn date_picker_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        DateInputFormat, DatePickerLevel, DatePickerMode, DateSelection, DatesProvider,
        date_picker, date_picker_input, stack,
    };

    let modes_theme = theme.clone();
    let modes = widget(move |scope| {
        let multiple = scope.cage(DateSelection::empty(DatePickerMode::Multiple));
        let range = scope.cage(DateSelection::empty(DatePickerMode::Range));
        stack()
            .theme(modes_theme.clone())
            .gap(20.)
            .fill((
                date_picker(
                    "showcase-date-picker-multiple",
                    DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
                )
                .theme(modes_theme.clone())
                .mode(DatePickerMode::Multiple)
                .value(multiple)
                .on_value_change(move |next| multiple.revise(|current| *current = next)),
                date_picker(
                    "showcase-date-picker-range",
                    DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
                )
                .theme(modes_theme.clone())
                .mode(DatePickerMode::Range)
                .value(range)
                .on_value_change(move |next| range.revise(|current| *current = next)),
            ))
            .show_in(scope)
    });

    let columns_theme = theme.clone();
    let columns = widget(move |scope| {
        let value = scope.cage(DateSelection::empty(DatePickerMode::Range));
        date_picker(
            "showcase-date-picker-columns",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(columns_theme.clone())
        .mode(DatePickerMode::Range)
        .value(value)
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .number_of_columns(2)
        .columns_to_scroll(2)
        .on_value_change(move |next| value.revise(|current| *current = next))
        .show_in(scope)
    });

    let input_theme = theme.clone();
    let input = widget(move |scope| {
        let value = scope.cage(DateSelection::empty(DatePickerMode::Range));
        date_picker_input(
            "showcase-date-picker-input-columns",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(input_theme.clone())
        .name("Travel dates")
        .trigger_label("Choose travel dates")
        .mode(DatePickerMode::Range)
        .value(value)
        .number_of_columns(2)
        .columns_to_scroll(1)
        .on_value_change(move |next| value.revise(|current| *current = next))
        .show_in(scope)
    });

    let formats_theme = theme.clone();
    let formats = stack().theme(formats_theme.clone()).gap(12.).fill((
        date_picker_input(
            "showcase-date-picker-format-single",
            DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap()),
        )
        .theme(formats_theme.clone())
        .name("Day first appointment")
        .trigger_label("Choose day first appointment")
        .display_format(DateInputFormat::DayMonthYear)
        .default_value(DateSelection::parse_iso(DatePickerMode::Single, "2026-11-04").unwrap()),
        date_picker_input(
            "showcase-date-picker-format-multiple",
            DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap()),
        )
        .theme(formats_theme.clone())
        .name("Month first dates")
        .trigger_label("Choose month first dates")
        .mode(DatePickerMode::Multiple)
        .display_format(DateInputFormat::MonthDayYear)
        .default_value(
            DateSelection::parse_iso(DatePickerMode::Multiple, "2026-11-04,2026-11-06").unwrap(),
        ),
        date_picker_input(
            "showcase-date-picker-format-range",
            DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap()),
        )
        .theme(formats_theme)
        .name("Day first range")
        .trigger_label("Choose day first range")
        .mode(DatePickerMode::Range)
        .display_format(DateInputFormat::DayMonthYear)
        .default_value(
            DateSelection::parse_iso(DatePickerMode::Range, "2026-11-04/2026-11-06").unwrap(),
        ),
    ));

    let calendar_rows_theme = theme.clone();
    let calendar_rows = stack().theme(calendar_rows_theme.clone()).gap(20.).fill((
        date_picker(
            "showcase-date-picker-week-numbers",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(calendar_rows_theme.clone())
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .with_week_numbers(true),
        date_picker(
            "showcase-date-picker-hidden-weekdays",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(calendar_rows_theme)
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .hide_weekdays(true),
    ));

    let levels_theme = theme.clone();
    let levels = stack().theme(levels_theme.clone()).gap(20.).fill((
        date_picker(
            "showcase-date-picker-levels",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(levels_theme.clone())
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .default_level(DatePickerLevel::Decade),
        date_picker(
            "showcase-date-picker-month-level-only",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(levels_theme)
        .name("Month-only date picker")
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .max_level(DatePickerLevel::Month),
    ));

    vec![
        DetailSection {
            id: "date-picker-modes",
            title: "Multiple dates and ranges",
            note: "Single, multiple and range modes use the same controlled selection protocol.",
            example: Box::new(modes),
            source: ".mode(DatePickerMode::Multiple)\n.value(value)\n.on_value_change(...)\n\n.mode(DatePickerMode::Range)",
        },
        DetailSection {
            id: "date-picker-columns",
            title: "Number of columns",
            note: "Consecutive month columns share one range selection and one month anchor. Outer navigation advances the configured number of columns.",
            example: Box::new(columns),
            source: ".mode(DatePickerMode::Range)\n.number_of_columns(2)\n.columns_to_scroll(2)",
        },
        DetailSection {
            id: "date-picker-input-columns",
            title: "Multiple columns in DatePickerInput",
            note: "The popup input forwards the same bounded multi-column calendar behavior.",
            example: Box::new(input),
            source: ".mode(DatePickerMode::Range)\n.number_of_columns(2)\n.columns_to_scroll(1)",
        },
        DetailSection {
            id: "date-picker-value-format",
            title: "Value format",
            note: "Readonly picker inputs can use finite ISO, day-first or month-first presentation without changing their typed DateSelection value.",
            example: Box::new(formats),
            source: ".display_format(DateInputFormat::DayMonthYear)",
        },
        DetailSection {
            id: "date-picker-levels",
            title: "Month, year and decade levels",
            note: "The header switches between the real day, month and year grids. default_level chooses the initial grid and max_level limits upward navigation.",
            example: Box::new(levels),
            source: ".default_level(DatePickerLevel::Decade)\n\n.max_level(DatePickerLevel::Month)",
        },
        DetailSection {
            id: "date-picker-calendar-rows",
            title: "Week numbers and weekday labels",
            note: "ISO week numbers add a passive table column. Weekday labels can be hidden without removing any of the 42 real day controls.",
            example: Box::new(calendar_rows),
            source: ".with_week_numbers(true)\n\n.hide_weekdays(true)",
        },
    ]
}

fn tree_detail_nodes() -> Vec<sapid::TreeNode> {
    use sapid::{TreeKind, TreeNode};
    vec![
        TreeNode::new("src", "src").kind(TreeKind::Branch),
        TreeNode::new("components", "components")
            .parent("src")
            .kind(TreeKind::Branch),
        TreeNode::new("button", "Button.rs").parent("components"),
        TreeNode::new("input", "Input.rs").parent("components"),
        TreeNode::new("main", "main.rs").parent("src"),
        TreeNode::new("tests", "tests").kind(TreeKind::Branch),
        TreeNode::new("button-test", "button_test.rs").parent("tests"),
        TreeNode::new("readme", "README.md"),
    ]
}

fn tree_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    use sapid::{
        TreeChoiceModel, TreeChoicePolicy, TreeChoices, TreeKind, TreeModel, TreeNode,
        TreeSelectionMode, button, expanded_tree_branches, filter_tree_nodes, group,
        move_tree_node, stack, text_field, tree,
    };
    use std::collections::BTreeSet;

    let controlled_theme = theme.clone();
    let controlled = widget(move |scope| {
        let expanded = scope.cage(BTreeSet::from(["src".to_owned()]));
        let selected = scope.cage(BTreeSet::from(["main".to_owned()]));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.0)
            .fill((
                tree("showcase-tree-controlled")
                    .theme(controlled_theme.clone())
                    .name("Controlled project tree")
                    .nodes(tree_detail_nodes())
                    .expanded(expanded)
                    .selected(selected)
                    .on_expanded_change(move |next| expanded.revise(|old| *old = next))
                    .on_selection_change(move |next| selected.revise(|old| *old = next)),
                glory::ui::text()
                    .class("showcase-tree-controlled-value")
                    .content(scope.bond(move || {
                        format!(
                            "Selected: {}",
                            selected
                                .get()
                                .iter()
                                .next()
                                .cloned()
                                .unwrap_or_else(|| "none".into())
                        )
                    })),
            ))
            .show_in(scope)
    });

    let expansion_theme = theme.clone();
    let expansion = widget(move |scope| {
        let nodes = tree_detail_nodes();
        let all = expanded_tree_branches(&nodes).expect("static tree data");
        let expanded = scope.cage(BTreeSet::new());
        stack()
            .theme(expansion_theme.clone())
            .gap(8.0)
            .fill((
                group().theme(expansion_theme.clone()).gap(8.0).fill((
                    button()
                        .theme(expansion_theme.clone())
                        .label("Expand all")
                        .on_press(move || expanded.revise(|value| *value = all.clone())),
                    button()
                        .theme(expansion_theme.clone())
                        .label("Collapse all")
                        .on_press(move || expanded.revise(BTreeSet::clear)),
                )),
                tree("showcase-tree-expansion")
                    .theme(expansion_theme.clone())
                    .name("Bulk expansion tree")
                    .nodes(nodes)
                    .expanded(expanded)
                    .on_expanded_change(move |next| expanded.revise(|old| *old = next)),
            ))
            .show_in(scope)
    });

    let checked_theme = theme.clone();
    let checked = widget(move |scope| {
        let nodes = tree_detail_nodes();
        let expanded = expanded_tree_branches(&nodes).expect("static tree data");
        let model = TreeChoiceModel::new(
            TreeModel::new(nodes.clone()).expect("static tree data"),
            TreeChoicePolicy::DescendantLeaves,
            true,
        )
        .expect("valid cascading policy");
        let choices = scope.cage(TreeChoices::default());
        let checked_model = model.clone();
        let checked = scope.bond(move || {
            checked_model
                .checked(&choices.get())
                .expect("current checked state")
        });
        tree("showcase-tree-checked")
            .theme(checked_theme.clone())
            .name("Cascading checked tree")
            .nodes(nodes)
            .default_expanded(expanded)
            .selection_mode(TreeSelectionMode::Multiple)
            .checked(checked)
            .on_selection_intent(move |key| {
                choices.revise(|current| {
                    if let Ok(next) = model.toggle(current, &key) {
                        *current = next;
                    }
                })
            })
            .show_in(scope)
    });

    let strict_theme = theme.clone();
    let strict = widget(move |scope| {
        let nodes = tree_detail_nodes();
        let expanded = expanded_tree_branches(&nodes).expect("static tree data");
        let model = TreeChoiceModel::new(
            TreeModel::new(nodes.clone()).expect("static tree data"),
            TreeChoicePolicy::AnyNode,
            true,
        )
        .expect("valid independent policy");
        let choices = scope.cage(TreeChoices::default());
        let checked_model = model.clone();
        let checked = scope.bond(move || {
            checked_model
                .checked(&choices.get())
                .expect("current checked state")
        });
        tree("showcase-tree-strict")
            .theme(strict_theme.clone())
            .name("Independent checked tree")
            .nodes(nodes)
            .default_expanded(expanded)
            .selection_mode(TreeSelectionMode::Multiple)
            .checked(checked)
            .on_selection_intent(move |key| {
                choices.revise(|current| {
                    if let Ok(next) = model.toggle(current, &key) {
                        *current = next;
                    }
                })
            })
            .show_in(scope)
    });

    let lazy_theme = theme.clone();
    let lazy = widget(move |scope| {
        let data = scope.cage(vec![
            TreeNode::new("workspace", "Workspace").kind(TreeKind::Branch),
            TreeNode::new("remote", "Remote package")
                .parent("workspace")
                .kind(TreeKind::Lazy),
        ]);
        let status = scope.cage("Expand Remote package to load children".to_owned());
        stack()
            .theme(lazy_theme.clone())
            .gap(8.0)
            .fill((
                tree("showcase-tree-lazy")
                    .theme(lazy_theme.clone())
                    .name("Lazy loading tree")
                    .nodes(data)
                    .default_expanded(BTreeSet::from(["workspace".into()]))
                    .on_load_children(move |key| {
                        if key != "remote" {
                            return;
                        }
                        data.revise(|nodes| {
                            if nodes[1].kind == TreeKind::Lazy {
                                nodes[1].kind = TreeKind::Branch;
                                nodes.push(
                                    TreeNode::new("remote-index", "index.tsx").parent("remote"),
                                );
                            }
                        });
                        status.revise(|value| *value = "Loaded Remote package".into());
                    }),
                glory::ui::text()
                    .class("showcase-tree-lazy-status")
                    .content(status),
            ))
            .show_in(scope)
    });

    let filter_theme = theme.clone();
    let filter = widget(move |scope| {
        let query = scope.cage(String::new());
        let filtered = scope.bond(move || {
            filter_tree_nodes(&tree_detail_nodes(), &query.get()).expect("bounded filter query")
        });
        let expanded = scope.bond(move || {
            expanded_tree_branches(&filtered.get()).expect("filtered tree remains valid")
        });
        let empty = scope.bond(move || filtered.with(Vec::is_empty));
        stack()
            .theme(filter_theme.clone())
            .gap(8.0)
            .fill((
                text_field()
                    .theme(filter_theme.clone())
                    .name("Filter tree nodes")
                    .placeholder("Search files")
                    .value(query)
                    .on_value_change(move |next| query.revise(|old| *old = next)),
                tree("showcase-tree-filter")
                    .theme(filter_theme.clone())
                    .name("Filtered project tree")
                    .nodes(filtered)
                    .expanded(expanded),
                when(empty, || {
                    glory::ui::text()
                        .class("showcase-tree-empty")
                        .content("Nothing found")
                }),
            ))
            .show_in(scope)
    });

    let drag_theme = theme.clone();
    let drag_demo = widget(move |scope| {
        let data = scope.cage(tree_detail_nodes());
        let expanded = expanded_tree_branches(&data.get()).expect("static tree data");
        let status = scope.cage("Drag a row before, inside or after another row".to_owned());
        tree("showcase-tree-drag")
            .theme(drag_theme.clone())
            .name("Reorderable project tree")
            .nodes(data)
            .default_expanded(expanded)
            .allow_drop(|proposal| proposal.target != "readme")
            .on_drag_drop(move |proposal| {
                let next = move_tree_node(&data.get(), &proposal);
                if let Ok(next) = next {
                    status.revise(|value| {
                        *value = format!(
                            "Moved {} {:?} {}",
                            proposal.dragged, proposal.position, proposal.target
                        )
                    });
                    data.revise(|current| *current = next);
                }
            })
            .show_in(scope)?;
        glory::ui::text()
            .class("showcase-tree-drag-status")
            .content(status)
            .show_in(scope)
    });

    let labels = tree("showcase-tree-labels")
        .theme(theme.clone())
        .name("Custom file tree")
        .nodes(tree_detail_nodes())
        .default_expanded(BTreeSet::from(["src".into(), "components".into()]))
        .file_icons(true)
        .render_label(move |node| {
            group().theme(theme.clone()).gap(6.0).fill((
                glory::ui::text().content(node.with(|node| node.label.clone())),
                sapid::badge()
                    .theme(theme.clone())
                    .content(node.with(|node| match node.kind {
                        TreeKind::Leaf => "file".to_owned(),
                        TreeKind::Branch | TreeKind::Lazy => "folder".to_owned(),
                    })),
            ))
        });

    vec![
        DetailSection {
            id: "tree-controlled",
            title: "Controlled expansion and selection",
            note: "Expansion and selection are application-owned values. Keyboard, pointer and typeahead proposals use the same controlled callbacks.",
            example: Box::new(controlled),
            source: ".expanded(expanded)\n.selected(selected)\n.on_expanded_change(...)\n.on_selection_change(...)",
        },
        DetailSection {
            id: "tree-expansion",
            title: "Expand and collapse all",
            note: "expanded_tree_branches derives the finite set of current branch keys; no component-side visited-node registry is retained.",
            example: Box::new(expansion),
            source: "let all = expanded_tree_branches(&nodes)?;\nexpanded.revise(|value| *value = all);",
        },
        DetailSection {
            id: "tree-checked",
            title: "Cascading checked state",
            note: "DescendantLeaves toggles current enabled leaves and derives checked or mixed parent state in one bounded postorder pass.",
            example: Box::new(checked),
            source: "TreeChoicePolicy::DescendantLeaves\n\n.checked(model.checked(&choices)?)\n.on_selection_intent(...)",
        },
        DetailSection {
            id: "tree-strict",
            title: "Independent checked state",
            note: "AnyNode provides strict independent toggles: checking a branch does not implicitly change its descendants.",
            example: Box::new(strict),
            source: "TreeChoicePolicy::AnyNode\n\n.checked(model.checked(&choices)?)",
        },
        DetailSection {
            id: "tree-lazy",
            title: "Async loading contract",
            note: "Opening a lazy branch emits one explicit load request. The application publishes the loaded declaration without hidden I/O in the widget.",
            example: Box::new(lazy),
            source: ".on_load_children(move |key| {\n    data.revise(|nodes| publish_children(nodes, key));\n})",
        },
        DetailSection {
            id: "tree-filter",
            title: "External search and filtering",
            note: "filter_tree_nodes is a bounded pure utility that retains every matching node's ancestor chain; the search field remains ordinary application state.",
            example: Box::new(filter),
            source: "let filtered = filter_tree_nodes(&nodes, &query)?;\nlet expanded = expanded_tree_branches(&filtered)?;",
        },
        DetailSection {
            id: "tree-drag-drop",
            title: "Drag and drop",
            note: "Pointer drops propose Before, Inside or After. allow_drop can reject a target, and move_tree_node publishes a validated finite declaration.",
            example: Box::new(drag_demo),
            source: ".allow_drop(|drop| drop.target != \"readme\")\n.on_drag_drop(move |drop| {\n    data.revise(|nodes| *nodes = move_tree_node(nodes, &drop)?);\n})",
        },
        DetailSection {
            id: "tree-render-label",
            title: "Custom labels and connecting lines",
            note: "render_label mounts typed Widget content with stable keyed Owners. File icons and real branch guide elements remain separate finite presentation facets.",
            example: Box::new(labels),
            source: ".file_icons(true)\n.render_label(|node| group().fill((text_label, kind_badge)))",
        },
    ]
}

pub fn detail_sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match id {
        "accordion" | "tabs" | "timeline" => super::showcase_composite_docs::sections(id, theme),
        "anchor" | "burger" | "nav-link" | "pagination" | "breadcrumbs" => {
            super::showcase_navigation_docs::sections(id, theme)
        }
        "menubar" => super::showcase_menubar::sections(theme),
        "app-shell" => super::showcase_shell::sections(theme),
        "color-input" => color_input_sections(theme),
        "color-picker" => color_picker_sections(theme),
        "modal" => modal_sections(theme),
        "slider" => slider_sections(theme),
        "range-slider" => range_slider_sections(theme),
        "progress" | "ring-progress" | "semi-circle-progress" => progress_sections(theme),
        "autocomplete" => autocomplete_sections(theme),
        "select" => select_sections(theme),
        "multi-select" => multi_select_sections(theme),
        "pills-input" => pills_input_sections(theme),
        "tags-input" => tags_input_sections(theme),
        "time-input" => time_input_sections(theme),
        "date-picker" | "date-picker-input" => date_picker_sections(theme),
        "time-picker" => time_picker_sections(theme),
        "date-time-picker" | "inline-date-time-picker" => date_time_picker_sections(theme),
        "tree" => tree_sections(theme),
        _ => Vec::new(),
    }
}

pub fn detail_navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "accordion" | "tabs" | "timeline" => super::showcase_composite_docs::navigation(id),
        "anchor" | "burger" | "nav-link" | "pagination" | "breadcrumbs" => {
            super::showcase_navigation_docs::navigation(id)
        }
        "menubar" => super::showcase_menubar::navigation(),
        "app-shell" => vec![
            ("Layout regions", "#shell-regions"),
            ("Responsive navigation", "#shell-responsive"),
            ("Collapsed navigation", "#shell-collapsed"),
        ],
        "date-picker" | "date-picker-input" => vec![
            ("Multiple and range", "#date-picker-modes"),
            ("Number of columns", "#date-picker-columns"),
            ("Input columns", "#date-picker-input-columns"),
            ("Value format", "#date-picker-value-format"),
            ("Levels", "#date-picker-levels"),
            ("Week numbers", "#date-picker-calendar-rows"),
        ],
        "color-input" => vec![
            ("Controlled", "#controlled"),
            ("Loading state", "#loading"),
            ("Formats", "#formats"),
            ("Preserve invalid input", "#preserve-invalid-input"),
            ("Committed changes", "#change-end"),
            ("Disable free input", "#disable-free-input"),
            ("With swatches", "#with-swatches"),
            ("Close on swatch", "#close-dropdown-on-swatch-click"),
            ("Hide dropdown", "#hide-dropdown"),
            ("Eye dropper", "#eye-dropper"),
            ("Eye dropper icon", "#eye-dropper-icon"),
            ("Left/right sections", "#left-right-sections"),
            ("Error", "#error"),
            ("Success", "#success"),
            ("Disabled", "#disabled"),
            ("Read only", "#read-only"),
        ],
        "color-picker" => vec![
            ("With swatches", "#picker-swatches"),
            ("Swatches only", "#picker-swatches-only"),
            ("Keyboard precision", "#picker-precision"),
            ("Formatted form value", "#picker-form-value"),
        ],
        "modal" => vec![
            ("Centered modal", "#modal-centered"),
            ("Full-screen modal", "#modal-full-screen"),
        ],
        "slider" => vec![
            ("Controlled value", "#controlled"),
            ("Committed changes", "#change-end"),
            ("Bounds and step", "#bounds-and-step"),
            ("Marks", "#marks"),
        ],
        "range-slider" => vec![
            ("Controlled range", "#controlled"),
            ("Bounds and gap", "#range-gap"),
            ("Marks", "#range-marks"),
            ("Maximum and inverted", "#range-advanced"),
            ("Domain", "#range-domain"),
            ("Control labels", "#range-labels"),
            ("Collision behavior", "#range-collision"),
        ],
        "progress" | "ring-progress" | "semi-circle-progress" => vec![
            ("Geometry", "#progress-geometry"),
            ("Compound sections", "#progress-sections"),
            ("Vertical orientation", "#progress-vertical"),
            ("Semicircle direction", "#progress-semicircle-direction"),
            ("Semicircle labels", "#progress-semicircle-labels"),
            ("Ring gaps", "#progress-ring-gaps"),
            ("Ring angles", "#progress-ring-angles"),
            ("Ring caps and label", "#progress-ring-caps-label"),
            ("States and values", "#progress-states"),
        ],
        "autocomplete" => vec![
            ("Controlled value", "#autocomplete-controlled"),
            ("Loading", "#autocomplete-loading"),
            ("First match", "#autocomplete-first"),
            ("Select on blur", "#autocomplete-blur"),
            ("Groups and clear", "#autocomplete-data"),
            ("Visible limit", "#autocomplete-limit"),
        ],
        "select" => vec![
            ("Controlled and clearable", "#select-controlled"),
            ("Loading", "#select-loading"),
            ("Option content", "#select-content"),
            ("Dropdown geometry", "#select-geometry"),
            ("Searchable composition", "#select-searchable"),
        ],
        "multi-select" => vec![
            ("Controlled values", "#multi-select-controlled"),
            ("Loading", "#multi-select-loading"),
            ("Groups and limit", "#multi-select-data"),
        ],
        "pills-input" => vec![
            ("Controlled pills", "#pills-controlled"),
            ("Loading", "#pills-loading"),
            ("Commit policies", "#pills-policies"),
        ],
        "tags-input" => vec![
            ("Controlled state", "#tags-controlled"),
            ("Loading", "#tags-loading"),
            ("Limits and clear", "#tags-limits-clear"),
            ("Accept on blur", "#tags-blur"),
            ("Duplicates and split", "#tags-parsing"),
        ],
        "time-input" => vec![
            ("Usage", "#time-input-usage"),
            ("Controlled", "#time-input-controlled"),
            ("Format and seconds", "#time-input-precision"),
            ("Placeholder", "#time-input-placeholder"),
            ("Control states", "#time-input-states"),
        ],
        "time-picker" => vec![
            ("Usage", "#time-picker-usage"),
            ("Controlled and clearable", "#time-picker-controlled"),
            ("Format", "#time-picker-format"),
            ("Duration", "#time-picker-duration"),
            ("Duration digits", "#time-picker-duration-digits"),
            ("Paste complete times", "#time-picker-paste"),
            ("Time controls dropdown", "#time-picker-dropdown"),
            ("Bounds and steps", "#time-picker-steps"),
            ("Time grid", "#time-picker-grid"),
            ("Presets", "#time-picker-presets"),
            ("Grouped presets", "#time-picker-grouped-presets"),
            ("Controlled popup", "#time-picker-controlled-popup"),
            ("Labels and focus", "#time-picker-labels-focus"),
            ("Popup layout", "#time-picker-popup-layout"),
            ("States", "#time-picker-states"),
        ],
        "date-time-picker" | "inline-date-time-picker" => vec![
            ("Usage", "#date-time-picker-usage"),
            ("Seconds", "#date-time-picker-seconds"),
            ("Presets", "#date-time-picker-presets"),
            ("TimePicker configuration", "#date-time-picker-time-options"),
            ("Controlled and clearable", "#date-time-picker-controlled"),
            ("Inline picker", "#date-time-picker-inline"),
            ("Calendar controls", "#date-time-picker-calendar-controls"),
            ("Date and time range", "#date-time-picker-range"),
            ("Inline range", "#date-time-picker-inline-range"),
            ("States", "#date-time-picker-states"),
        ],
        "tree" => vec![
            ("Controlled state", "#tree-controlled"),
            ("Expand all", "#tree-expansion"),
            ("Cascading checks", "#tree-checked"),
            ("Independent checks", "#tree-strict"),
            ("Lazy loading", "#tree-lazy"),
            ("Search", "#tree-filter"),
            ("Drag and drop", "#tree-drag-drop"),
            ("Custom labels", "#tree-render-label"),
        ],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glory::render::{Event, Props};

    #[test]
    fn shell_and_menubar_documentation_sections_mount_real_widgets_and_retire() {
        for (slug, count) in [
            ("app-shell", 3),
            ("menubar", 7),
            ("anchor", 2),
            ("burger", 1),
            ("nav-link", 2),
            ("breadcrumbs", 2),
            ("pagination", 3),
            ("accordion", 4),
            ("tabs", 5),
            ("timeline", 3),
        ] {
            let sections = detail_sections(slug, sapid::Theme::blue_light());
            assert_eq!(sections.len(), count);
            assert_eq!(detail_navigation(slug).len(), count);
            for section in sections {
                assert!(section.source.contains("pub fn demo("));
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|error| panic!("{}: {error}", section.id));
                assert!(!app.tree().is_empty(), "{}", section.id);
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0, "{}", section.id);
            }
        }
    }

    #[test]
    fn clean_examples_build_real_widgets_and_retire_their_owned_state() {
        for id in [
            "button",
            "text-input",
            "checkbox",
            "collapse",
            "accordion",
            "tabs",
            "modal",
            "drawer",
            "combobox-popover",
        ] {
            let app = App::memory();
            app.mount(demo(id, sapid::Theme::blue_light())).unwrap();
            assert!(!app.tree().is_empty(), "empty demo: {id}");
            if id == "button" {
                let mut pending = app.tree().roots().to_vec();
                let mut button = None;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    if matches!(node.descriptor.view().props, Props::Button(p) if p.label == "Filled button")
                    {
                        button = Some(id);
                    }
                    pending.extend(node.children.iter().copied());
                }
                app.dispatch(button.unwrap(), Event::Activate).unwrap();
                let mut pending = app.tree().roots().to_vec();
                let mut updated = false;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    updated |= matches!(node.descriptor.view().props, Props::Text(p) if p.content == "Clicked 1 times");
                    pending.extend(node.children.iter().copied());
                }
                assert!(
                    updated,
                    "the displayed counter must respond to native activation"
                );
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired demo: {id}");
            assert_eq!(app.runtime_stats().tasks, 0, "retired tasks: {id}");
        }
    }

    #[test]
    fn color_input_documentation_builds_every_interactive_detail_section() {
        let sections = detail_sections("color-input", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "controlled",
                "loading",
                "formats",
                "preserve-invalid-input",
                "change-end",
                "disable-free-input",
                "with-swatches",
                "close-dropdown-on-swatch-click",
                "hide-dropdown",
                "eye-dropper",
                "eye-dropper-icon",
                "left-right-sections",
                "error",
                "success",
                "disabled",
                "read-only",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount ColorInput section {}: {error}", section.id)
            });
            assert!(!app.tree().is_empty(), "empty section: {}", section.id);
            if section.id == "formats" {
                let mut pending = app.tree().roots().to_vec();
                let mut inputs = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    inputs += usize::from(matches!(node.descriptor.view().props, Props::Input(_)));
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(inputs, 6, "every documented color format must render");
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn color_picker_documentation_builds_swatches_and_precision_sections() {
        let sections = detail_sections("color-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "picker-swatches",
                "picker-swatches-only",
                "picker-precision",
                "picker-form-value"
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!(
                    "failed to mount ColorPicker section {}: {error}",
                    section.id
                )
            });
            let mut pending = app.tree().roots().to_vec();
            let mut swatches = 0;
            let mut planes = 0;
            while let Some(id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(id).unwrap();
                swatches += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-color-picker-swatch"),
                );
                planes += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-color-plane"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "picker-swatches-only" {
                assert_eq!(swatches, 7);
                assert_eq!(planes, 0);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn modal_documentation_builds_centered_and_full_screen_layers() {
        let sections = detail_sections("modal", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            ["modal-centered", "modal-full-screen"]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut full_screen = 0;
            while let Some(id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(id).unwrap();
                full_screen += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-dialog-full-screen"),
                );
                pending.extend(node.children.iter().copied());
            }
            assert_eq!(full_screen, usize::from(section.id == "modal-full-screen"));
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn slider_documentation_builds_feature_sections_and_real_marks() {
        let sections = detail_sections("slider", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            ["controlled", "change-end", "bounds-and-step", "marks"]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            if section.id == "marks" {
                let mut pending = app.tree().roots().to_vec();
                let mut marks = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    marks += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-slider-mark"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(marks, 8);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn range_slider_documentation_builds_atomic_range_and_marks_sections() {
        let sections = detail_sections("range-slider", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "controlled",
                "range-gap",
                "range-marks",
                "range-advanced",
                "range-domain",
                "range-labels",
                "range-collision",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            if matches!(
                section.id,
                "range-marks" | "range-advanced" | "range-domain"
            ) {
                let mut pending = app.tree().roots().to_vec();
                let mut marks = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    marks += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-range-slider-mark"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(
                    marks,
                    match section.id {
                        "range-marks" => 5,
                        "range-advanced" => 3,
                        "range-domain" => 2,
                        _ => unreachable!(),
                    }
                );
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn progress_documentation_builds_geometry_sections_and_accessibility_states() {
        for id in ["progress", "ring-progress", "semi-circle-progress"] {
            let sections = detail_sections(id, sapid::Theme::blue_light());
            assert_eq!(
                sections
                    .iter()
                    .map(|section| section.id)
                    .collect::<Vec<_>>(),
                [
                    "progress-geometry",
                    "progress-sections",
                    "progress-vertical",
                    "progress-semicircle-direction",
                    "progress-semicircle-labels",
                    "progress-ring-gaps",
                    "progress-ring-angles",
                    "progress-ring-caps-label",
                    "progress-states",
                ]
            );
            for section in sections {
                let app = App::memory();
                app.mount(section.example).unwrap();
                if section.id == "progress-sections" {
                    let mut pending = app.tree().roots().to_vec();
                    let mut parts = 0;
                    while let Some(node_id) = pending.pop() {
                        let tree = app.tree();
                        let node = tree.node(node_id).unwrap();
                        parts += usize::from(
                            node.descriptor
                                .view()
                                .classes
                                .iter()
                                .any(|class| class == "sapid-progress-section"),
                        );
                        pending.extend(node.children.iter().copied());
                    }
                    assert_eq!(parts, 6);
                }
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
            }
        }
    }

    #[test]
    fn autocomplete_documentation_builds_all_behavior_sections_and_limits_results() {
        let sections = detail_sections("autocomplete", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "autocomplete-controlled",
                "autocomplete-loading",
                "autocomplete-first",
                "autocomplete-blur",
                "autocomplete-data",
                "autocomplete-limit",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut visible_options = 0;
            let mut clear_buttons = 0;
            let mut loading_indicators = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                visible_options += usize::from(
                    node.descriptor
                        .view()
                        .semantics
                        .as_ref()
                        .is_some_and(|value| value.option.is_some() && !value.hidden),
                );
                clear_buttons += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-autocomplete-clear"),
                );
                loading_indicators += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-autocomplete-loading"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "autocomplete-limit" {
                assert_eq!(visible_options, 5);
            }
            if section.id == "autocomplete-data" {
                assert_eq!(clear_buttons, 1);
            }
            if section.id == "autocomplete-loading" {
                assert_eq!(loading_indicators, 2);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn select_documentation_builds_controlled_loading_content_geometry_and_search_composition() {
        let sections = detail_sections("select", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "select-controlled",
                "select-loading",
                "select-content",
                "select-geometry",
                "select-searchable",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount Select section {}: {error}", section.id)
            });
            let mut pending = app.tree().roots().to_vec();
            let mut loading = 0;
            let mut descriptions = 0;
            let mut checks = 0;
            let mut visible_options = 0;
            let mut listbox_heights = Vec::new();
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                let view = node.descriptor.view();
                loading += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-select-loading"),
                );
                descriptions += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-select-option-description"),
                );
                checks += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-select-option-check"),
                );
                visible_options += usize::from(
                    view.semantics
                        .as_ref()
                        .is_some_and(|value| value.option.is_some() && !value.hidden),
                );
                if view.role == Some(glory::render::Role::Listbox) {
                    listbox_heights.push(view.style.max_height);
                }
                pending.extend(node.children.iter().copied());
            }
            match section.id {
                "select-loading" => assert_eq!(loading, 1),
                "select-content" => {
                    assert_eq!(descriptions, 2);
                    assert_eq!(checks, 2);
                }
                "select-geometry" => {
                    assert!(listbox_heights.contains(&Some(glory::style::Limit::Px(180.0))))
                }
                "select-searchable" => assert_eq!(visible_options, 2),
                _ => {}
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn tree_documentation_builds_controlled_checked_lazy_filter_drag_and_custom_label_sections() {
        let sections = detail_sections("tree", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "tree-controlled",
                "tree-expansion",
                "tree-checked",
                "tree-strict",
                "tree-lazy",
                "tree-filter",
                "tree-drag-drop",
                "tree-render-label",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount Tree section {}: {error}", section.id)
            });
            let mut pending = app.tree().roots().to_vec();
            let mut tree_items = 0;
            let mut checked_items = 0;
            let mut draggable_items = 0;
            let mut custom_badges = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                let view = node.descriptor.view();
                tree_items += usize::from(view.role == Some(glory::render::Role::TreeItem));
                checked_items += usize::from(
                    view.semantics
                        .as_ref()
                        .is_some_and(|value| value.tree_checked.is_some()),
                );
                draggable_items += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-tree-draggable"),
                );
                custom_badges +=
                    usize::from(view.classes.iter().any(|class| class == "sapid-badge"));
                pending.extend(node.children.iter().copied());
            }
            assert!(tree_items > 0, "empty Tree section: {}", section.id);
            if matches!(section.id, "tree-checked" | "tree-strict") {
                assert_eq!(checked_items, tree_items);
            }
            if section.id == "tree-drag-drop" {
                assert_eq!(draggable_items, tree_items);
            }
            if section.id == "tree-render-label" {
                assert_eq!(custom_badges, tree_items);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn multi_select_documentation_builds_controlled_loading_and_data_sections() {
        let sections = detail_sections("multi-select", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "multi-select-controlled",
                "multi-select-loading",
                "multi-select-data",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut clear_buttons = 0;
            let mut loading_indicators = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                clear_buttons += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-multi-select-clear"),
                );
                loading_indicators += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-combobox-loading"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "multi-select-loading" {
                assert_eq!(loading_indicators, 2);
            }
            if section.id != "multi-select-loading" {
                assert_eq!(clear_buttons, 1);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn tags_input_documentation_builds_controlled_loading_clear_blur_and_parsing_sections() {
        let sections = detail_sections("tags-input", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "tags-controlled",
                "tags-loading",
                "tags-limits-clear",
                "tags-blur",
                "tags-parsing",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut clear_buttons = 0;
            let mut loading_indicators = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                clear_buttons += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-tags-input-clear"),
                );
                loading_indicators += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-tags-input-loading"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "tags-loading" {
                assert_eq!(loading_indicators, 2);
            }
            if section.id == "tags-limits-clear" {
                assert_eq!(clear_buttons, 1);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
        assert_eq!(
            detail_sections("pills-input", sapid::Theme::blue_light())
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            ["pills-controlled", "pills-loading", "pills-policies"]
        );
    }

    #[test]
    fn time_input_documentation_builds_usage_controlled_format_placeholder_and_state_sections() {
        let sections = detail_sections("time-input", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "time-input-usage",
                "time-input-controlled",
                "time-input-precision",
                "time-input-placeholder",
                "time-input-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut inputs = 0;
            let mut placeholders = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                inputs += usize::from(matches!(node.descriptor.view().props, Props::Input(_)));
                placeholders += usize::from(
                    node.descriptor
                        .view()
                        .semantics
                        .as_ref()
                        .and_then(|value| value.placeholder.as_ref())
                        .is_some(),
                );
                pending.extend(node.children.iter().copied());
            }
            assert!(inputs > 0);
            if matches!(section.id, "time-input-usage" | "time-input-placeholder") {
                assert!(placeholders > 0);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn time_picker_documentation_builds_input_grid_preset_popup_and_state_sections() {
        let sections = detail_sections("time-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "time-picker-usage",
                "time-picker-controlled",
                "time-picker-format",
                "time-picker-duration",
                "time-picker-duration-digits",
                "time-picker-paste",
                "time-picker-dropdown",
                "time-picker-steps",
                "time-picker-grid",
                "time-picker-presets",
                "time-picker-grouped-presets",
                "time-picker-controlled-popup",
                "time-picker-labels-focus",
                "time-picker-popup-layout",
                "time-picker-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount TimePicker section {}: {error}", section.id)
            });
            assert!(!app.tree().is_empty(), "empty section: {}", section.id);
            if section.id == "time-picker-controlled" {
                let mut pending = app.tree().roots().to_vec();
                let mut clear = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    clear += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-time-picker-clear"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(clear, 1);
            }
            if matches!(
                section.id,
                "time-picker-presets"
                    | "time-picker-grouped-presets"
                    | "time-picker-controlled-popup"
                    | "time-picker-popup-layout"
            ) {
                let mut pending = app.tree().roots().to_vec();
                let mut trigger = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    trigger += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-time-picker-preset-trigger"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(trigger, 1);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn date_picker_documentation_builds_modes_columns_and_input_columns() {
        let sections = detail_sections("date-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "date-picker-modes",
                "date-picker-columns",
                "date-picker-input-columns",
                "date-picker-value-format",
                "date-picker-levels",
                "date-picker-calendar-rows",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let grid_count = app
                .tree()
                .roots()
                .iter()
                .copied()
                .flat_map(|root| {
                    let mut nodes = vec![root];
                    let mut output = Vec::new();
                    while let Some(id) = nodes.pop() {
                        let tree = app.tree();
                        let node = tree.node(id).unwrap();
                        nodes.extend(node.children.iter().copied());
                        output.push(id);
                    }
                    output
                })
                .filter(|id| {
                    app.tree()
                        .node(*id)
                        .unwrap()
                        .descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-calendar-grid")
                })
                .count();
            assert!(grid_count >= 2);
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn date_time_picker_documentation_builds_all_official_behavior_sections() {
        let sections = detail_sections("date-time-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "date-time-picker-usage",
                "date-time-picker-seconds",
                "date-time-picker-presets",
                "date-time-picker-time-options",
                "date-time-picker-controlled",
                "date-time-picker-inline",
                "date-time-picker-calendar-controls",
                "date-time-picker-range",
                "date-time-picker-inline-range",
                "date-time-picker-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example)
                .unwrap_or_else(|error| panic!("failed to mount {}: {error}", section.id));
            assert!(!app.tree().is_empty(), "empty section: {}", section.id);
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}
