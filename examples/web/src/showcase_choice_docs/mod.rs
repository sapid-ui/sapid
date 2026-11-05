use super::showcase_examples::DetailSection;
mod checkbox_group;
mod checkbox_labels;
mod checkbox_states;
mod chip_modes;
mod chip_states;
mod fieldset_disabled;
mod fieldset_layout;
mod radio_controlled;
mod radio_orientation;
mod radio_states;
mod rating_controlled;
mod rating_states;
mod rating_symbols;
mod segments_controlled;
mod segments_layout;
mod segments_width;
mod switch_group;
mod switch_labels;
mod switch_sizes;
mod switch_states;

pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident, $id:literal, $title:literal, $note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    match id {
        "switch" => vec![
            section!(
                switch_sizes,
                "switch-sizes",
                "Sizes and track marks",
                "Five sizes preserve a genuine switch control, keyboard activation and the same accepted checked state. with_marks draws passive check/cross marks inside the track; it does not introduce extra focus stops or controls."
            ),
            section!(
                switch_labels,
                "switch-labels",
                "Label position, description and error",
                "label_position places the caption before or after the track. Clicking either the track or caption activates the same native input. Field connects its label, description, required and error state to that input. The decorative caption never duplicates its accessible name."
            ),
            section!(
                switch_states,
                "switch-states",
                "Controlled refusal, read-only and disabled",
                "checked supplies the accepted value and on_checked_change proposes a replacement. Declining a change restores native checkedness. Read-only remains focusable while refusing changes; disabled prevents normal activation. Internal behavior runs before generic observers."
            ),
            section!(
                switch_group,
                "switch-group",
                "Independent values and disabled groups",
                "Each switch owns an independent boolean. A Fieldset names and disables the group without clearing values or replacing inputs. The shared enabled source also dims each portable track. Application enable/disable actions belong outside the disabled group."
            ),
        ],
        "rating" => vec![
            section!(
                rating_controlled,
                "rating-half",
                "Half values, controlled selection and clearing",
                "Whole and Half are the finite supported precisions. Each selectable half is a genuine radio value with an exact number/name. Clearable adds an explicit Clear rating radio for zero; activating a selected symbol does not implicitly clear it. Controlled callbacks can refuse proposals. Keyboard navigation uses the same group and keeps one normal Tab stop."
            ),
            section!(
                rating_states,
                "rating-states",
                "Display-only, read-only, disabled and sizes",
                "Display-only exposes a named passive score group without radios or successful form controls. Read-only keeps its controls focusable while refusing changes; disabled prevents normal interaction. Compare all five sizes without rebuilding a live rating on every prop change. Fractional values outside the configured Whole/Half domain are rejected rather than silently rounding an application source."
            ),
            section!(
                rating_symbols,
                "rating-symbols",
                "Custom empty and filled symbols",
                "empty_symbol and filled_symbol take static decorative GraphicProps. They share the existing symbol box, half clipping, accepted score, hover preview and radio semantics. The example includes its complete heart drawing helper. Drawings are validated before controls mount, with at most 256 commands and finite geometry; named, animated or transitioning graphics are rejected. Arbitrary interactive child widgets are not symbol declarations."
            ),
        ],
        "segmented-control" => vec![
            section!(
                segments_width,
                "segments-width",
                "Equal full width and content width",
                "full_width is reactive. In a horizontal group it distributes the available row equally; the default keeps content widths. The same native controls, selection and focus survive switching modes. Explicit segment styles can override the provided geometry. A full-width control belongs in a parent with a real bounded width."
            ),
            section!(
                segments_layout,
                "segments-layout",
                "Vertical layout, disabled items and long labels",
                "Vertical orientation changes both layout and directional navigation. Disabled members are skipped. Full-width long labels share the same row and may wrap within each segment, retaining their complete accessible names. Small changes the compact theme size. This remains a single value choice, with no tab panels or separate application view controller."
            ),
            section!(
                segments_controlled,
                "segments-controlled",
                "Controlled refusal and read-only selection",
                "value is a read-only accepted key. Declining a proposed Grid choice repairs native checkedness and retains List. Enabling acceptance permits a real change. Read-only keeps the same accepted value and blocks selection commands. Each group has its own stable namespace, roving focus and natural label activation."
            ),
        ],
        "checkbox" => vec![
            section!(
                checkbox_group,
                "checkbox-group",
                "Indeterminate state and select all",
                "CheckboxGroup derives its real mixed state from enabled members. Initially only Email is selected. All available channels selects Email and SMS together, skips the disabled Push option, and toggles them off together on the next activation. Item identities stay stable. A standalone MixedCheckbox uses CheckState::Mixed; ordinary activation proposes a boolean selection. The group is bounded to 1024 explicit mounted members."
            ),
            section!(
                checkbox_labels,
                "checkbox-labels",
                "Label position, description and error",
                "The label is the native activation surface, including wrapped text. A horizontal Field places its linked label before the input while preserving the same native label/control relationship. Field adds required, description and error relationships to the actual checkbox. A display error does not silently set its checked value."
            ),
            section!(
                checkbox_states,
                "checkbox-states",
                "Controlled refusal, read-only and disabled",
                "checked is a read-only source. Until acceptance is enabled, the callback declines changes and the native input returns to the accepted checked state. Click the label or press Space on the input. Read-only remains focusable but refuses changes; disabled prevents normal interaction. Changing an external source remains an application action."
            ),
        ],
        "radio" => vec![
            section!(
                radio_orientation,
                "radio-orientation",
                "Vertical and horizontal groups",
                "Radio is a named value inside exactly one RadioGroup. Orientation changes both layout and directional keyboard navigation. Arrow keys select the next available member, Home/End reach boundaries, and one current member participates in normal Tab navigation. The groups have independent stable namespaces and selection."
            ),
            section!(
                radio_controlled,
                "radio-controlled",
                "Controlled selection and unavailable members",
                "value is the accepted key; callbacks propose a replacement without mutating it. Declining a proposal repairs the native checked state. Enabling acceptance permits selection. Navigation skips the disabled middle member. Selecting the already checked radio keeps it selected; use an application clear action with an empty key when an optional choice is needed."
            ),
            section!(
                radio_states,
                "radio-states",
                "Descriptions, read-only and errors",
                "Field connects descriptions and validation errors to the real group/member controls. Read-only keeps the accepted selection and blocks selection commands. required participates in form validity; an error message is explicit application validation. Group size and theme apply consistently to members. form_name is an explicit successful-control name, independent of the accessible label."
            ),
        ],
        "chip" => vec![
            section!(
                chip_modes,
                "chip-modes",
                "Single, multiple and clearing selection",
                "Single mode is a real radio group: choosing another chip replaces the key and clicking the selected chip does not clear it. The explicit Clear action writes the application's empty key. Multiple mode is a real checkbox group: activating an already checked chip removes that key, including the final selection. value/on_value_change and values/on_values_change are separate typed contracts; callbacks may decline proposals."
            ),
            section!(
                chip_states,
                "chip-states",
                "Long labels, read-only and disabled",
                "Chip is a value selection control, with native checkbox/radio semantics and a compact pill surface. Button is an action; Checkbox presents the same boolean choice with its normal check indicator. Long labels wrap within their parent. Read-only retains focus and selection; disabled chips are skipped by normal interaction. Named groups retain distinct item keys and keyboard behavior."
            ),
        ],
        "fieldset" => vec![
            section!(
                fieldset_disabled,
                "fieldset-disabled",
                "Disable an entire fieldset",
                "disabled applies to descendant controls through the shared fieldset semantics, including text, checkbox, switch and button. Toggle the control outside the fieldset to re-enable them. The same editor, draft and checked values survive; no subtree is rebuilt. The legend names the group. Keep enable/disable actions outside a disabled group."
            ),
            section!(
                fieldset_layout,
                "fieldset-layout",
                "Nested groups, borders and spacing",
                "A nested Fieldset has its own legend and finite group identity. Ordinary layout styles control padding, gaps and borders; removing a border does not remove the legend or group semantics. Child Field labels remain attached to their actual editors. A disabled outer group also makes nested controls unavailable; a child cannot opt itself back in."
            ),
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "switch" => vec![
            ("Sizes and marks", "#switch-sizes"),
            ("Labels and feedback", "#switch-labels"),
            ("Controlled and unavailable", "#switch-states"),
            ("Independent group", "#switch-group"),
        ],
        "rating" => vec![
            ("Half values and clearing", "#rating-half"),
            ("States and sizes", "#rating-states"),
            ("Custom symbols", "#rating-symbols"),
        ],
        "segmented-control" => vec![
            ("Width modes", "#segments-width"),
            ("Layout and long labels", "#segments-layout"),
            ("Controlled and readonly", "#segments-controlled"),
        ],
        "checkbox" => vec![
            ("Indeterminate and select all", "#checkbox-group"),
            ("Labels and error", "#checkbox-labels"),
            ("Controlled and unavailable", "#checkbox-states"),
        ],
        "radio" => vec![
            ("Orientation", "#radio-orientation"),
            ("Controlled selection", "#radio-controlled"),
            ("Descriptions and errors", "#radio-states"),
        ],
        "chip" => vec![
            ("Modes and clearing", "#chip-modes"),
            ("Labels and unavailable", "#chip-states"),
        ],
        "fieldset" => vec![
            ("Disable descendants", "#fieldset-disabled"),
            ("Nesting and spacing", "#fieldset-layout"),
        ],
        _ => Vec::new(),
    }
}
