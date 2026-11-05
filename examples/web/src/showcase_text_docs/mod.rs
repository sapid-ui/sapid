use super::showcase_examples::DetailSection;
mod controlled;
mod labels;
mod order;
mod sections;
mod sizes;
mod states;
mod textarea_autosize;
mod textarea_fixed;
mod textarea_states;
mod variants;
pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
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
    match id {
        "input" => vec![
            section!(
                labels,
                "input-labels",
                "Labels, descriptions and validation",
                "Field supplies a real linked label, description, required marker and error relationship. A placeholder is only a hint. Invalid and success style are independent of the value; error takes precedence. The actual editor is TextField, built with text_field()."
            ),
            section!(
                sections,
                "input-sections",
                "Reserved sections, clear action and loading",
                "Decorations and named compact actions occupy reserved input padding. Section widths are finite 20–200 pixels. Clear changes application state; loading does not itself disable or make an editor read-only. Labels and relations still point at the actual editor."
            ),
            section!(
                sizes,
                "input-sizes",
                "Five sizes",
                "Field size is inherited by its editor. Padding, font, line height and minimum height use the same theme. Explicit editor styles retain precedence; every example remains an ordinary named input."
            ),
            section!(
                order,
                "input-order",
                "Field ordering",
                "order takes each of Label, Description, Control and Error exactly once. Reordering the visual parts retains the actual label/control and description/error relationships. A disabled Field blocks its own control; a Fieldset supplies a descendant boundary."
            ),
        ],
        "text-input" => vec![
            section!(
                controlled,
                "text-controlled",
                "Controlled editing, read-only and disabled",
                "value is the accepted source. Declining on_value_change restores accepted text at the complete host packet boundary. Read-only blocks proposals while retaining focus; disabled blocks normal interaction. Program writes remain application actions. Changes to these states preserve the same editor and selection identity."
            ),
            section!(
                sections,
                "text-sections",
                "Left decoration and inset clear action",
                "The real editor reserves space for both sections. The named clear action occupies a 24-pixel button in the right slot, and uses the application's controlled value. It does not inject DOM state or replace the editor. Loading has an explicit side."
            ),
            section!(
                variants,
                "text-variants",
                "Default, filled and borderless styles",
                "Typed Style changes the presentation of the same TextField. Explicit background and border declarations demonstrate filled and borderless variants without a second controller or semantic wrapper. Theme defaults continue to supply unspecified properties."
            ),
            section!(
                states,
                "text-states",
                "Long hints and unavailable states",
                "Long placeholder text clips within the input instead of widening the page. Hints do not supply labels or values. Read-only retains selectable text; disabled retains its accepted value. Generic observers run only after accepted internal behavior."
            ),
        ],
        "textarea" => vec![
            section!(
                textarea_fixed,
                "textarea-fixed",
                "Fixed rows and ordered limits",
                "rows uses effective line height, padding and borders. min_rows and max_rows must be positive and ordered. Long content scrolls inside a fixed editor; Enter inserts a newline. These fixed-row declarations are portable to native Apace."
            ),
            section!(
                textarea_autosize,
                "textarea-autosize",
                "Autosize on HTML hosts",
                "autosize uses FieldSizing::Content, preserving soft wrapping and later width/font changes on supporting HTML hosts. Two minimum and five maximum rows bound this example. Native Apace explicitly rejects autosize; fixed rows are a separate supported path."
            ),
            section!(
                textarea_states,
                "textarea-states",
                "Controlled draft, errors and unavailable states",
                "Textarea shares TextField's revision, controlled refusal, Field and composition rules. Validation messages remain separate from text. Read-only keeps the draft available for selection; disabled prevents editing without clearing it. No extra JSON, formatting or submission controller is implied."
            ),
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "input" => vec![
            ("Labels and validation", "#input-labels"),
            ("Sections and loading", "#input-sections"),
            ("Sizes", "#input-sizes"),
            ("Field ordering", "#input-order"),
        ],
        "text-input" => vec![
            ("Controlled editing", "#text-controlled"),
            ("Inset sections", "#text-sections"),
            ("Visual variants", "#text-variants"),
            ("Hints and states", "#text-states"),
        ],
        "textarea" => vec![
            ("Fixed rows", "#textarea-fixed"),
            ("HTML autosize", "#textarea-autosize"),
            ("Draft and states", "#textarea-states"),
        ],
        _ => Vec::new(),
    }
}
