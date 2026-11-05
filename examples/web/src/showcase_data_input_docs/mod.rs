use super::showcase_examples::DetailSection;
mod file_limits;
mod file_multiple;
mod file_single;
mod file_states;
mod json_blur;
mod json_errors;
mod json_formats;
mod json_states;
mod mask_editing;
mod mask_patterns;
mod mask_states;
mod mask_values;
mod native_groups;
mod native_modes;
mod native_sizes;
mod native_states;
mod number_clamp;
mod number_formats;
mod number_sizes;
mod number_states;
mod number_steps;
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
        "file-input" => vec![
            section!(
                file_single,
                "file-single",
                "Single file summary and clearing",
                "The summary is the actual selected filename on the same real picker control. clearable adds an inset named action. User clearing emits on_files with an empty list; FileController::clear is silent programmatic clearing. Both revoke current host references. show_attachments(false) hides rows without manufacturing or prepopulating file rights."
            ),
            section!(
                file_multiple,
                "file-multiple",
                "Multiple files and long names",
                "multiple and max_files are separate limits: this example admits three files. The input shows the first name and the remaining count; attachment rows keep the full names and independently named removal actions. Long names clip inside the input and stay available in attachment rows. Cancelling a chooser retains the current selection."
            ),
            section!(
                file_limits,
                "file-limits",
                "Types, count and byte limits",
                "FileConfig explicitly bounds count, MIME/extension filters and per-file/total bytes. The host validates the whole result before publishing opaque FileToken references. MIME metadata is declared information, not content validation. Display names never become paths, and no read or preview starts from displaying the summary."
            ),
            section!(
                file_states,
                "file-states",
                "Disabled, cancelled and unavailable operations",
                "Disabled prevents choosing or user clearing. Cancellation and rejection preserve earlier accepted files. The host owns picker/read admission, cancellation and grants; clearing or retiring the control revokes current acceptance. Native hosts need an explicit file service. Model/provider tests and browser chooser automation do not establish OS dialog or real-device acceptance."
            ),
        ],
        "mask-input" => vec![
            section!(
                mask_patterns,
                "mask-patterns",
                "Telephone, date-shaped and code patterns",
                "# accepts an ASCII digit, A a letter and * an alphanumeric slot. Backslash escapes a literal. Telephone/date-shaped examples format characters; the date-shaped reference does not validate calendar dates. Patterns have at most 128 parts and 64 slots. Unicode digits and arbitrary format functions are outside this finite contract."
            ),
            section!(
                mask_values,
                "mask-values",
                "Raw values and displayed separators",
                "The public String contains slot characters only. The same editor displays separators. Program updates use raw values and remain silent; on_value_change proposes raw replacements. Placeholder is independent hint metadata and never becomes the value or accessible name. Empty raw values display an empty editor."
            ),
            section!(
                mask_editing,
                "mask-editing",
                "Replacement, deletion, paste and composition",
                "The real editor owns its UTF-16 selection. Replacement/deletion maps between displayed literals and raw slots; raw or explicitly formatted paste is accepted atomically. Invalid or overlong input is refused without filtering a suffix. Native composition stays unformatted until its terminal event; non-ASCII terminal text does not become ASCII business data."
            ),
            section!(
                mask_states,
                "mask-states",
                "Controlled refusal and current availability",
                "value is read-only accepted raw text. The application may refuse proposals and the same editor restores accepted display/selection. Read-only retains focus; disabled prevents editing. Current Field availability is checked within the host packet before typed callbacks and generic observers. Neither state creates a second hidden input."
            ),
        ],
        "native-select" => vec![
            section!(
                native_groups,
                "native-groups",
                "Native option groups and unavailable options",
                "This is the host's actual select with static bounded options and optgroups. Disabled groups disable their members; disabled options are skipped by normal native interaction. Placeholder is a native unselected prompt. Platform popup appearance and keys are provided by the host, without a competing combobox controller."
            ),
            section!(
                native_modes,
                "native-modes",
                "Single dropdown and multiple listbox",
                "Single mode takes Option<String> and on_value_change. Multiple mode takes Vec<String>, rows and on_values_change. Cardinality is explicit and mixed contracts are rejected. The visible listbox and its selection gestures remain native. Each option key is distinct from its successful form value."
            ),
            section!(
                native_states,
                "native-states",
                "Controlled selection, required and disabled",
                "Declining a proposal restores the accepted native selection without replacing options. Disabled retains selected keys and blocks commands, including a same-packet enabled update. Field supplies labels, required and errors to the actual select. readonly(true) is explicitly rejected because native select has no readonly contract."
            ),
            section!(
                native_sizes,
                "native-sizes",
                "Five native control sizes",
                "The theme supplies font, padding and minimum height to the real native select. Native option popup presentation remains platform-owned; custom popup children belong to Select/Combobox. Configuration and accepted selection retain stable physical option identities."
            ),
        ],
        "json-input" => vec![
            section!(
                json_formats,
                "json-formats",
                "Whitespace formats and exact lexemes",
                "PrettyTwo, PrettyFour and Compact edit whitespace while preserving key order, duplicate members, escape spelling and exact number lexemes. They never decode application numbers through floating point. Use the formatter button or opt into formatting on blur."
            ),
            section!(
                json_errors,
                "json-errors",
                "Editable errors, positions and budgets",
                "Syntax errors report a bounded message with line/column and remain editable. Field error relationships target the actual textarea. Output-budget failures preserve a valid original draft and have a separate format error. JsonLimits bounds bytes and depth; formatting does not grant schema validation or syntax highlighting."
            ),
            section!(
                json_blur,
                "json-blur",
                "Formatting on real focus loss",
                "format_on_blur(true) shares the same bounded formatter as the button. It proposes a change before on_format and generic blur observers. Invalid, composing, read-only and disabled drafts remain untouched. The default is false; show_formatter(false) can hide the button independently."
            ),
            section!(
                json_states,
                "json-states",
                "Controlled proposals and unavailable states",
                "value is read-only accepted text. Change and formatting callbacks may be refused by the application. Syntax checking, accepted formatting and Field updates preserve the actual editor. Read-only and disabled keep accepted text, and formatting respects current availability within the host packet. This editor uses fixed rows on all supported hosts."
            ),
        ],
        "number-input" => vec![
            section!(
                number_formats,
                "number-formats",
                "Explicit locale, grouping and exact precision",
                "Decimal arithmetic retains up to 18 fractional digits within the bounded exact domain. Locale is explicit, with no host discovery. User drafts preserve typed zeroes, signs and incomplete decimals; program Decimal values use normalized formatting. Grouping affects display, while accessible numeric coordinates remain lossy f64 values. Configurable fixed-scale padding is not provided."
            ),
            section!(
                number_clamp,
                "number-clamp",
                "Blur, strict and no-clamp policies",
                "NumberClampBehavior::Blur is the default: drafts may leave the range until Enter or actual focus loss. Strict refuses complete out-of-range user edits; empty/incomplete drafts remain editable. None keeps out-of-range values and marks their range validity. Step commands stay bounded in all modes. Controlled commits remain proposals."
            ),
            section!(
                number_steps,
                "number-steps",
                "Exact steps, page steps and optional controls",
                "Arrow and named chevron controls use exact decimal step; Page keys use page_step. Home/End use explicit endpoints. controls(false) removes buttons while keeping keyboard and spinbutton semantics. Caret navigation and native text composition remain ordinary editor operations."
            ),
            section!(
                number_states,
                "number-states",
                "Controlled drafts, read-only and disabled",
                "Both on_value_change and on_commit propose NumberValue values. Until acceptance is enabled, edits, steps and blur normalization retain the application's accepted quantity. Read-only blocks editing and steps while remaining focusable. Disabled prevents normal activation. Internal availability guards precede generic observers."
            ),
            section!(
                number_sizes,
                "number-sizes",
                "Five sizes and inset controls",
                "The spinbutton and compact two-button section share one bordered surface. Field size changes the editor geometry while the action section stays inside that surface. The examples fit a narrow parent without presenting the controls as full-size text buttons."
            ),
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "file-input" => vec![
            ("Single and clear", "#file-single"),
            ("Multiple and long names", "#file-multiple"),
            ("Filters and budgets", "#file-limits"),
            ("Availability and cancellation", "#file-states"),
        ],
        "mask-input" => vec![
            ("Patterns", "#mask-patterns"),
            ("Raw and display values", "#mask-values"),
            ("Editing and composition", "#mask-editing"),
            ("Controlled and unavailable", "#mask-states"),
        ],
        "native-select" => vec![
            ("Native groups", "#native-groups"),
            ("Single and multiple", "#native-modes"),
            ("Controlled and states", "#native-states"),
            ("Sizes", "#native-sizes"),
        ],
        "json-input" => vec![
            ("Formats", "#json-formats"),
            ("Errors and limits", "#json-errors"),
            ("Real blur", "#json-blur"),
            ("Controlled and unavailable", "#json-states"),
        ],
        "number-input" => vec![
            ("Format and precision", "#number-formats"),
            ("Clamping", "#number-clamp"),
            ("Stepping", "#number-steps"),
            ("Controlled and unavailable", "#number-states"),
            ("Sizes", "#number-sizes"),
        ],
        _ => Vec::new(),
    }
}
