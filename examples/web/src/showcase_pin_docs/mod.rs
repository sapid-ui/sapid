use super::showcase_examples::DetailSection;
mod geometry;
mod masking;
mod states;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "pin-geometry",
            title: "Length, padding and spacing",
            note: "length accepts 1–16; gap and cell_padding accept 0–32 finite CSS pixels. Each cell uses the real monospace zero-glyph width plus its padding. The editor uses the same font advance and letter spacing, with trailing editing space for its native caret. Wide codes have a local horizontal viewport. No cell owns a separate value, focus stop or mirrored code.",
            example: Box::new(geometry::demo(theme.clone())),
            source: include_str!("geometry.rs"),
        },
        DetailSection {
            id: "pin-masking",
            title: "Alphanumeric and reactive masking",
            note: "Digits accepts ASCII 0–9; AsciiAlphanumeric also accepts ASCII letters and preserves case. masked accepts static or Cage/Bond/Vary state. Changing it patches the purpose of the same native editor, preserving value, selection and composition. Password protection belongs to the host; decorative cells never contain a copy of the code.",
            example: Box::new(masking::demo(theme.clone())),
            source: include_str!("masking.rs"),
        },
        DetailSection {
            id: "pin-states",
            title: "Fields, errors and unavailable controls",
            note: "Field connects the real editor to its label, description, required and error state. readonly preserves the accepted code without editing; enabled(false) disables the native input. on_value_change proposes the complete String, on_complete follows only a newly accepted full user value, and on_reject reports bounded policy errors without code contents. Paste is validated atomically; no automatic submit or verification request is performed.",
            example: Box::new(states::demo(theme)),
            source: include_str!("states.rs"),
        },
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Length and spacing", "#pin-geometry"),
        ("Alphabet and masking", "#pin-masking"),
        ("Field states", "#pin-states"),
    ]
}
