use super::showcase_examples::DetailSection;
mod content;
mod controlled;
mod geometry;
mod loading;
mod searchable;
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
            "select-controlled",
            "Controlled and clearable",
            "value accepts Cage/Bond/Vary<Option<String>>. on_value_change proposes a known option key or None; this example explicitly accepts it. Clear is an independent inset button, and returns actual focus to the same combobox. Refused proposals preserve the last accepted value. Form submission uses the intrinsic successful Select proxy, separately from its accessible name."
        ),
        section!(
            loading,
            "select-loading",
            "Loading state",
            "loading adds an inline indicator and busy metadata without replacing or disabling the real combobox. The indicator occupies the disclosure slot. Applications own asynchronous loading, request cancellation and the option data; Select does not install I/O or infer a loading service."
        ),
        section!(
            content,
            "select-content",
            "Groups, option content and check position",
            "Open the field to inspect a labeled group and passive leading, description and trailing text. check_position selects a left, right or hidden selected mark. Declarations are bounded to 1024 options; disabled entries remain unavailable. Matching, active option and accepted selection remain separate states."
        ),
        section!(
            geometry,
            "select-geometry",
            "Dropdown height and placement",
            "Open this field to inspect a bounded 180px listbox placed above its trigger. max_dropdown_height accepts 80–4096 finite pixels. popup_placement uses the shared anchored layer contract. Long selected labels remain bounded; the optional browser stylesheet supplies ellipsis, while native layout clips without enabling an unsupported text-overflow style."
        ),
        section!(
            searchable,
            "select-searchable",
            "Searchable selection composition",
            "Editable search and filtering use the explicit Combobox composition. This example has Prefix filtering, a two-result limit, a loading indicator and an empty message. Focus the editor and change its search text. Plain Select retains its non-editable button combobox, native typeahead, arrows, Home/End and Escape behavior."
        ),
        section!(
            states,
            "select-states",
            "Field states and controlled refusal",
            "Field connects label, description, required and error state to the actual combobox. readonly suppresses value proposals while preserving the documented close/navigation policy; enabled(false) disables interaction. Turn acceptance off to compare a refused selection or clear proposal. The visible control and its accepted value remain stable."
        ),
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Controlled and clearable", "#select-controlled"),
        ("Loading", "#select-loading"),
        ("Option content", "#select-content"),
        ("Dropdown geometry", "#select-geometry"),
        ("Searchable composition", "#select-searchable"),
        ("Field states", "#select-states"),
    ]
}
