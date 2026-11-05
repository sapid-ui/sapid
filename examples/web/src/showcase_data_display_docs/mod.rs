mod avatar_fallbacks;
mod avatar_group;
mod avatar_recovery;
mod avatar_sizes;
mod badge_actions;
mod badge_sections;
mod badge_sizes;
mod badge_variants;
mod card_content;
mod card_sections;
mod card_surfaces;
mod data_content;
mod data_orientation;
mod data_spacing;
mod image_alternative;
mod image_dimensions;
mod image_fallback;
mod image_fit;
mod indicator_offsets;
mod indicator_pointer;
mod indicator_positions;
mod kbd_combinations;
mod kbd_platforms;
mod kbd_sizes;
mod number_affixes;
mod number_locales;
mod number_precision;
mod overflow_controlled;
mod overflow_retained;
mod overflow_width;
mod rolling_digits;
mod rolling_format;
mod rolling_motion;
mod spoiler_changes;
mod spoiler_controlled;
mod spoiler_threshold;
mod swatch_alpha;
mod swatch_controlled;
mod swatch_sizes;
mod theme_icon_actions;
mod theme_icon_sizes;
mod theme_icon_variants;
use super::showcase_examples::DetailSection;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "avatar" => vec![
            DetailSection {
                id: "avatar-fallbacks",
                title: "Photos, initials and vector fallback",
                note: "The accessible person name is independent of the photo. Initials are supplied explicitly; the vector fallback remains visible until the current image loads and returns after failure.",
                example: Box::new(avatar_fallbacks::demo(theme.clone())),
                source: include_str!("avatar_fallbacks.rs"),
            },
            DetailSection {
                id: "avatar-sizes",
                title: "Five sizes and two shapes",
                note: "The five theme sizes use 24, 30, 36, 42 and 50 pixel boxes. Circle and Square select the outline; the person name remains available in either shape.",
                example: Box::new(avatar_sizes::demo(theme.clone())),
                source: include_str!("avatar_sizes.rs"),
            },
            DetailSection {
                id: "avatar-group",
                title: "Overlapping group and overflow count",
                note: "Stable explicit member keys preserve avatar identities. max_visible and overlap bound the row; the overflow marker has a spoken count rather than pretending to be another person.",
                example: Box::new(avatar_group::demo(theme.clone())),
                source: include_str!("avatar_group.rs"),
            },
            DetailSection {
                id: "avatar-recovery",
                title: "Image failure and recovery",
                note: "Switch between an invalid image payload and a local photo. The same avatar and fallback survive current-request completion; stale completions cannot reveal an older photo.",
                example: Box::new(avatar_recovery::demo(theme.clone())),
                source: include_str!("avatar_recovery.rs"),
            },
        ],
        "badge" => vec![
            DetailSection {
                id: "badge-variants",
                title: "Variants and readable custom color",
                note: "Soft, Solid, Outline and Dot share a finite passive label. Solid chooses a contrasting foreground; a custom color controls the dot in Dot mode.",
                example: Box::new(badge_variants::demo(theme.clone())),
                source: include_str!("badge_variants.rs"),
            },
            DetailSection {
                id: "badge-sizes",
                title: "Compact sizes and long labels",
                note: "Caption metrics scale across the five theme sizes. A long label clips within its bounded box while its complete text remains in the semantic tree.",
                example: Box::new(badge_sizes::demo(theme.clone())),
                source: include_str!("badge_sizes.rs"),
            },
            DetailSection {
                id: "badge-sections",
                title: "Passive sections and reactive status",
                note: "Sections accept a finite Icon or up to 32 bytes of text. Updating the central label keeps its actual text node and vector section.",
                example: Box::new(badge_sections::demo(theme.clone())),
                source: include_str!("badge_sections.rs"),
            },
            DetailSection {
                id: "badge-actions",
                title: "Badges beside real actions",
                note: "Badge is passive. A real Button owns activation; its inline sections accept plain text badges. Rich boxed badges stay beside the action because intrinsic button content is deliberately finite.",
                example: Box::new(badge_actions::demo(theme.clone())),
                source: include_str!("badge_actions.rs"),
            },
        ],
        "card" => vec![
            DetailSection {
                id: "card-sections",
                title: "Header, body and footer",
                note: "Card exposes three explicit content slots and an optional semantic heading. Each slot accepts actual widgets; a footer can hold an independent action without making the entire card a button.",
                example: Box::new(card_sections::demo(theme.clone())),
                source: include_str!("card_sections.rs"),
            },
            DetailSection {
                id: "card-surfaces",
                title: "Border, shadow and radius",
                note: "Border and shadow are independent surface choices. Radius and padding are ordinary explicit Style fields, so no competing card layout or hidden section shorthand is required.",
                example: Box::new(card_surfaces::demo(theme.clone())),
                source: include_str!("card_surfaces.rs"),
            },
            DetailSection {
                id: "card-content",
                title: "Long content and independent actions",
                note: "The body grows with ordinary wrapping text. Changing project details retains the real action and heading rather than replacing the complete card.",
                example: Box::new(card_content::demo(theme.clone())),
                source: include_str!("card_content.rs"),
            },
        ],
        "color-swatch" => vec![
            DetailSection {
                id: "swatch-alpha",
                title: "Transparency and checkerboard",
                note: "A real finite checkerboard remains beneath transparent colors. Every passive swatch has an explicit text name; color alone is not the explanation.",
                example: Box::new(swatch_alpha::demo(theme.clone())),
                source: include_str!("swatch_alpha.rs"),
            },
            DetailSection {
                id: "swatch-sizes",
                title: "Sizes and rounded outlines",
                note: "side accepts a finite 8 to 128 pixel box. round chooses the circle; square swatches retain the theme radius and checkerboard clipping.",
                example: Box::new(swatch_sizes::demo(theme.clone())),
                source: include_str!("swatch_sizes.rs"),
            },
            DetailSection {
                id: "swatch-controlled",
                title: "Controlled selection and disabled state",
                note: "pressed is read-only. The callback proposes selection to the application; refusing a proposal keeps the current checkmark. Disabled swatches do not emit activation.",
                example: Box::new(swatch_controlled::demo(theme.clone())),
                source: include_str!("swatch_controlled.rs"),
            },
        ],
        "data-list" => vec![
            DetailSection {
                id: "data-orientation",
                title: "Horizontal and vertical descriptions",
                note: "Horizontal uses a label column and a flexible value column. Vertical stacks each term and definition. Both keep real description-list semantics and direct term/definition children.",
                example: Box::new(data_orientation::demo(theme.clone())),
                source: include_str!("data_orientation.rs"),
            },
            DetailSection {
                id: "data-spacing",
                title: "Label width, spacing and compact rows",
                note: "label_width and gap are explicit finite pixel values. Structural layout and zero definition margins survive unstyled; appearance remains independently themeable.",
                example: Box::new(data_spacing::demo(theme.clone())),
                source: include_str!("data_spacing.rs"),
            },
            DetailSection {
                id: "data-content",
                title: "Wrapping values and real widgets",
                note: "Definitions accept ordinary widgets, including text, badges and a real action. Long prose wraps within the flexible column; unbreakable identifiers need an explicit local overflow policy.",
                example: Box::new(data_content::demo(theme.clone())),
                source: include_str!("data_content.rs"),
            },
        ],
        "image" => vec![
            DetailSection {
                id: "image-fit",
                title: "Cover and contain",
                note: "The same local photo uses two explicit object-fit policies. Cover crops to fill the box; Contain preserves the complete photo and leaves the remaining surface visible.",
                example: Box::new(image_fit::demo(theme.clone())),
                source: include_str!("image_fit.rs"),
            },
            DetailSection {
                id: "image-dimensions",
                title: "Intrinsic proportion and rounded corners",
                note: "A full-width photo can keep its natural aspect ratio. A separate square crop uses explicit dimensions and radius; the semantic image itself remains the resource owner.",
                example: Box::new(image_dimensions::demo(theme.clone())),
                source: include_str!("image_dimensions.rs"),
            },
            DetailSection {
                id: "image-fallback",
                title: "A bounded fallback and current request recovery",
                note: "fallback_src retries once after the current primary fails. A valid local fallback and meaningful alternative text keep the example useful without a network image service.",
                example: Box::new(image_fallback::demo(theme.clone())),
                source: include_str!("image_fallback.rs"),
            },
            DetailSection {
                id: "image-alternative",
                title: "Informative and decorative images",
                note: "Informative images carry a content description. decorative explicitly omits the image from the accessibility tree when adjacent text supplies the same meaning.",
                example: Box::new(image_alternative::demo(theme.clone())),
                source: include_str!("image_alternative.rs"),
            },
        ],
        "indicator" => vec![
            DetailSection {
                id: "indicator-positions",
                title: "Nine anchor positions",
                note: "The marker anchors to the content box. All nine physical positions use the same independent content slot; a centered axis remains centered when offset changes.",
                example: Box::new(indicator_positions::demo(theme.clone())),
                source: include_str!("indicator_positions.rs"),
            },
            DetailSection {
                id: "indicator-offsets",
                title: "Offsets, labels and color",
                note: "Positive offsets inset the chosen edge; negative offsets move outward. Diameter is explicit and does not grow with label text, so choose a box that fits the short count.",
                example: Box::new(indicator_offsets::demo(theme.clone())),
                source: include_str!("indicator_offsets.rs"),
            },
            DetailSection {
                id: "indicator-pointer",
                title: "Overlapping markers preserve activation",
                note: "The passive marker does not intercept pointer input. Try the action underneath the centered marker; hiding the marker also leaves the same independent action enabled.",
                example: Box::new(indicator_pointer::demo(theme.clone())),
                source: include_str!("indicator_pointer.rs"),
            },
        ],
        "kbd" => vec![
            DetailSection {
                id: "kbd-combinations",
                title: "Readable key combinations",
                note: "Use separate keycaps with ordinary separators. Kbd is a passive hint and does not register a shortcut or steal keyboard input.",
                example: Box::new(kbd_combinations::demo(theme.clone())),
                source: include_str!("kbd_combinations.rs"),
            },
            DetailSection {
                id: "kbd-platforms",
                title: "Explicit platform names",
                note: "Windows, Mac and Linux labels are selected explicitly. Platform choice does not inspect the host or create operating-system key bindings.",
                example: Box::new(kbd_platforms::demo(theme.clone())),
                source: include_str!("kbd_platforms.rs"),
            },
            DetailSection {
                id: "kbd-sizes",
                title: "Theme sizes and ordinary editing",
                note: "Keycap typography follows the theme size in light and dark schemes. The adjacent input remains an ordinary controlled editor; the hint has no activation handler.",
                example: Box::new(kbd_sizes::demo(theme.clone())),
                source: include_str!("kbd_sizes.rs"),
            },
        ],
        "number-formatter" => vec![
            DetailSection {
                id: "number-locales",
                title: "Three explicit locale policies",
                note: "EnUs, DeDe and ZhCn are finite deterministic format policies, independent of host locale. No claim is made for arbitrary Intl locale or numbering-system support.",
                example: Box::new(number_locales::demo(theme.clone())),
                source: include_str!("number_locales.rs"),
            },
            DetailSection {
                id: "number-precision",
                title: "Grouping, precision and percentages",
                note: "Fraction limits control deterministic rounding and padding. Percent multiplies by 100 before formatting; grouping can be disabled explicitly.",
                example: Box::new(number_precision::demo(theme.clone())),
                source: include_str!("number_precision.rs"),
            },
            DetailSection {
                id: "number-affixes",
                title: "Currency and reactive literal affixes",
                note: "prefix and suffix are bounded literal strings around the complete formatted value. They do not alter the numeric source, currency policy or sign placement. Invalid reactive affixes preserve the last accepted output.",
                example: Box::new(number_affixes::demo(theme.clone())),
                source: include_str!("number_affixes.rs"),
            },
        ],
        "overflow-list" => vec![
            DetailSection {
                id: "overflow-width",
                title: "Container width and automatic overflow",
                note: "Resize the actual container with the checkbox. Current host measurements move the same item widgets into the More popup; the deterministic initial prefix is only the first SSR shape.",
                example: Box::new(overflow_width::demo(theme.clone())),
                source: include_str!("overflow_width.rs"),
            },
            DetailSection {
                id: "overflow-controlled",
                title: "Controlled prefix and popup",
                note: "visible_count fixes the inline prefix. open and its callback form a read-only controlled proposal; the application can refuse opening and later accept it without recreating item owners.",
                example: Box::new(overflow_controlled::demo(theme.clone())),
                source: include_str!("overflow_controlled.rs"),
            },
            DetailSection {
                id: "overflow-retained",
                title: "Retained editing and keyboard access",
                note: "The same input moves between the inline row and popup. Its explicit finite width makes measurement independent of available width. Focus and normal text editing remain on the real editor.",
                example: Box::new(overflow_retained::demo(theme.clone())),
                source: include_str!("overflow_retained.rs"),
            },
        ],
        "rolling-number" => vec![
            DetailSection {
                id: "rolling-digits",
                title: "Increasing, decreasing and digit count changes",
                note: "Digit presentation follows increases, decreases, carries and sign changes. The semantic value is the complete current formatted result, independent of the decorative outgoing digits.",
                example: Box::new(rolling_digits::demo(theme.clone())),
                source: include_str!("rolling_digits.rs"),
            },
            DetailSection {
                id: "rolling-format",
                title: "Locale, currency and precision",
                note: "RollingNumber shares the finite number formatting policy. Group separators, signs and currency remain part of the final semantic string rather than individual interactive cells.",
                example: Box::new(rolling_format::demo(theme.clone())),
                source: include_str!("rolling_format.rs"),
            },
            DetailSection {
                id: "rolling-motion",
                title: "Motion preference, pause and immediate output",
                note: "animated false and zero duration display the final value immediately. The browser also honors prefers-reduced-motion; pause stops presentation without changing the numeric source. No per-frame application Cage is created.",
                example: Box::new(rolling_motion::demo(theme.clone())),
                source: include_str!("rolling_motion.rs"),
            },
        ],
        "spoiler" => vec![
            DetailSection {
                id: "spoiler-threshold",
                title: "Threshold and short content",
                note: "The current measured text height decides whether a trigger is necessary. Short content does not display an inactive expansion action; long content clips at the explicit max_height.",
                example: Box::new(spoiler_threshold::demo(theme.clone())),
                source: include_str!("spoiler_threshold.rs"),
            },
            DetailSection {
                id: "spoiler-controlled",
                title: "Controlled expansion and custom labels",
                note: "expanded is read-only and the callback proposes a new state. The application may refuse the change. The trigger keeps aria-expanded and its controlled content relationship in sync.",
                example: Box::new(spoiler_controlled::demo(theme.clone())),
                source: include_str!("spoiler_controlled.rs"),
            },
            DetailSection {
                id: "spoiler-changes",
                title: "Content changes and immediate presentation",
                note: "Changing text remeasures the same viewport and updates trigger availability. Spoiler currently settles immediately with no transition, including reduced-motion environments; max_height is not an animation duration.",
                example: Box::new(spoiler_changes::demo(theme.clone())),
                source: include_str!("spoiler_changes.rs"),
            },
        ],
        "theme-icon" => vec![
            DetailSection {
                id: "theme-icon-sizes",
                title: "Container size and independent glyph size",
                note: "Theme size selects a 24, 30, 36, 42 or 50 pixel container. icon_size independently controls the finite drawing, preserving its physical node when a valid reactive size changes.",
                example: Box::new(theme_icon_sizes::demo(theme.clone())),
                source: include_str!("theme_icon_sizes.rs"),
            },
            DetailSection {
                id: "theme-icon-variants",
                title: "Variants, radius and explicit glyph color",
                note: "Soft, Solid and Outline use theme surfaces. color overrides the glyph stroke; explicit Style background and radius configure the container. This color contract is distinct from Mantine palette-name props.",
                example: Box::new(theme_icon_variants::demo(theme.clone())),
                source: include_str!("theme_icon_variants.rs"),
            },
            DetailSection {
                id: "theme-icon-actions",
                title: "Passive presentation and a real action",
                note: "ThemeIcon is passive and cannot receive press handlers. ActionIcon supplies real button semantics, keyboard activation and an accessible name when an icon should perform an action.",
                example: Box::new(theme_icon_actions::demo(theme.clone())),
                source: include_str!("theme_icon_actions.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "avatar" => vec![
            ("Photos, initials and vector fallback", "#avatar-fallbacks"),
            ("Five sizes and two shapes", "#avatar-sizes"),
            ("Overlapping group and overflow count", "#avatar-group"),
            ("Image failure and recovery", "#avatar-recovery"),
        ],
        "badge" => vec![
            ("Variants and readable custom color", "#badge-variants"),
            ("Compact sizes and long labels", "#badge-sizes"),
            ("Passive sections and reactive status", "#badge-sections"),
            ("Badges beside real actions", "#badge-actions"),
        ],
        "card" => vec![
            ("Header, body and footer", "#card-sections"),
            ("Border, shadow and radius", "#card-surfaces"),
            ("Long content and independent actions", "#card-content"),
        ],
        "color-swatch" => vec![
            ("Transparency and checkerboard", "#swatch-alpha"),
            ("Sizes and rounded outlines", "#swatch-sizes"),
            (
                "Controlled selection and disabled state",
                "#swatch-controlled",
            ),
        ],
        "data-list" => vec![
            ("Horizontal and vertical descriptions", "#data-orientation"),
            ("Label width, spacing and compact rows", "#data-spacing"),
            ("Wrapping values and real widgets", "#data-content"),
        ],
        "image" => vec![
            ("Cover and contain", "#image-fit"),
            (
                "Intrinsic proportion and rounded corners",
                "#image-dimensions",
            ),
            (
                "A bounded fallback and current request recovery",
                "#image-fallback",
            ),
            ("Informative and decorative images", "#image-alternative"),
        ],
        "indicator" => vec![
            ("Nine anchor positions", "#indicator-positions"),
            ("Offsets, labels and color", "#indicator-offsets"),
            (
                "Overlapping markers preserve activation",
                "#indicator-pointer",
            ),
        ],
        "kbd" => vec![
            ("Readable key combinations", "#kbd-combinations"),
            ("Explicit platform names", "#kbd-platforms"),
            ("Theme sizes and ordinary editing", "#kbd-sizes"),
        ],
        "number-formatter" => vec![
            ("Three explicit locale policies", "#number-locales"),
            ("Grouping, precision and percentages", "#number-precision"),
            ("Currency and reactive literal affixes", "#number-affixes"),
        ],
        "overflow-list" => vec![
            ("Container width and automatic overflow", "#overflow-width"),
            ("Controlled prefix and popup", "#overflow-controlled"),
            ("Retained editing and keyboard access", "#overflow-retained"),
        ],
        "rolling-number" => vec![
            (
                "Increasing, decreasing and digit count changes",
                "#rolling-digits",
            ),
            ("Locale, currency and precision", "#rolling-format"),
            (
                "Motion preference, pause and immediate output",
                "#rolling-motion",
            ),
        ],
        "spoiler" => vec![
            ("Threshold and short content", "#spoiler-threshold"),
            (
                "Controlled expansion and custom labels",
                "#spoiler-controlled",
            ),
            (
                "Content changes and immediate presentation",
                "#spoiler-changes",
            ),
        ],
        "theme-icon" => vec![
            (
                "Container size and independent glyph size",
                "#theme-icon-sizes",
            ),
            (
                "Variants, radius and explicit glyph color",
                "#theme-icon-variants",
            ),
            (
                "Passive presentation and a real action",
                "#theme-icon-actions",
            ),
        ],
        _ => Vec::new(),
    }
}
