use super::showcase_examples::DetailSection;
mod content;
mod controlled;
mod loading;
mod sizes;
mod states;
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
            controlled,
            "password-controlled",
            "Controlled value and visibility",
            "value and visible are independent read-only Cage/Bond/Vary sources. Their callbacks propose changes; declining a proposal preserves the accepted value or visibility. default_value and default_visible configure local state. The visibility action changes the purpose of the same native editor without copying its value or advancing its edit revision."
        ),
        section!(
            content,
            "password-sections",
            "Left and custom right sections",
            "The default eye is an inset icon button with an accessible Show password/Hide password name. A left_section keeps that action; a caller-supplied right_section replaces it completely. Configure section widths to reserve editor space. Custom visibility UI owns its own proposal handling. Passive drawings do not replace the field label."
        ),
        section!(
            loading,
            "password-loading",
            "Loading indicators and visibility",
            "Loading occupies the configured side without replacing the editor. A right-side indicator temporarily hides the default eye; left-side loading leaves it available. Loading does not submit or validate a password. Changing loading state preserves the draft and editor identity."
        ),
        section!(
            states,
            "password-states",
            "Field metadata and unavailable states",
            "Field provides label, description, required, error and disabled state. Read-only prevents editing and still permits a visibility change; disabled prevents both editing and the eye action. visibility_labels localizes both action names. Hidden passwords retain the protected native role and are omitted from accessibility values; revealing intentionally changes the editor to ordinary text."
        ),
        section!(
            sizes,
            "password-sizes",
            "Sizes and icon geometry",
            "All five sizes share the same inset action and reserve space within the input. The eye drawing scales inside its hit target; it is not a separate full-size text button. Typed style customizations compose with the theme. unstyled opts out of themed paint and dimensions, so the caller must style the input and its custom section appropriately."
        ),
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Controlled value and visibility", "#password-controlled"),
        ("Left and custom right sections", "#password-sections"),
        ("Loading indicators and visibility", "#password-loading"),
        ("Field metadata and unavailable states", "#password-states"),
        ("Sizes and icon geometry", "#password-sizes"),
    ]
}
