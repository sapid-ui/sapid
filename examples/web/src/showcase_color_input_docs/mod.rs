use super::showcase_examples::DetailSection;
mod close_on_swatch;
mod commit_demo;
mod controlled;
mod disable_free_input;
mod draft;
mod eye_dropper;
mod eye_dropper_icon;
mod formats;
mod hidden_dropdown;
mod loading;
mod palette_only;
mod sections;
mod state_disabled;
mod state_error;
mod state_readonly;
mod state_success;
mod swatches;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident,$id:literal,$title:literal,$note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    vec![
        section!(
            palette_only,
            "swatches-only",
            "Controlled swatches-only dropdown",
            "show_picker(false) removes the continuous plane and sliders. Supplying swatches retains a palette popup, including the same controlled open/on_open_change contract. A preview action reopens it without rebuilding the editor. Selecting a swatch proposes the color and, when configured, closing. Declining visibility changes retains the accepted source. With neither picker nor swatches there is no dropdown, so popup state configuration is rejected."
        ),
        section!(
            controlled,
            "controlled",
            "Controlled",
            "The input value is owned by application state and the current formatted value is rendered beside the control."
        ),
        section!(
            loading,
            "loading",
            "Loading state",
            "Loading is reactive, keeps the live value intact and can be toggled without rebuilding the input."
        ),
        section!(
            formats,
            "formats",
            "Formats",
            "Hex, RGB and HSL formats support opaque and alpha variants. Alpha formats add the opacity control to the picker."
        ),
        section!(
            draft,
            "preserve-invalid-input",
            "Preserve invalid input",
            "Disable blur correction when the application needs to keep an unfinished or invalid draft visible."
        ),
        section!(
            commit_demo,
            "change-end",
            "Committed changes",
            "The commit callback runs when text editing finishes or a picker interaction is committed, independently of continuous changes."
        ),
        section!(
            disable_free_input,
            "disable-free-input",
            "Disable free input",
            "Typing is read-only while the continuous picker remains available for deliberate color selection."
        ),
        section!(
            swatches,
            "with-swatches",
            "With swatches",
            "A finite palette is rendered below the continuous picker and publishes the same formatted value as direct manipulation."
        ),
        section!(
            close_on_swatch,
            "close-dropdown-on-swatch-click",
            "Close dropdown on color swatch click",
            "Selecting a swatch can atomically commit the value and close the popup."
        ),
        section!(
            hidden_dropdown,
            "hide-dropdown",
            "Hide dropdown",
            "The editable color field can be used without mounting a picker popup."
        ),
        section!(
            eye_dropper,
            "eye-dropper",
            "Eye dropper",
            "The application explicitly supplies the asynchronous sampling service; the component never requests ambient browser or OS authority."
        ),
        section!(
            eye_dropper_icon,
            "eye-dropper-icon",
            "Change eye dropper icon",
            "The sampler keeps an accessible label while its visible icon is supplied by the application."
        ),
        section!(
            sections,
            "left-right-sections",
            "Left and right sections",
            "Custom left and right sections replace the default preview area when supplied."
        ),
        section!(
            state_error,
            "error",
            "Error",
            "Error metadata, message association and invalid state are applied to the real input."
        ),
        section!(
            state_success,
            "success",
            "Success",
            "Success styling is independent of the current parsed color and yields to an active error."
        ),
        section!(
            state_disabled,
            "disabled",
            "Disabled",
            "Disabled state blocks editing, popup actions and the optional sampler service."
        ),
        section!(
            state_readonly,
            "read-only",
            "Read only",
            "Read-only state preserves the value while blocking text, picker and sampler changes."
        ),
    ]
}
