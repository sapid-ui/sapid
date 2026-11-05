use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::field("textarea-autosize").theme(theme.clone()).label("Autosizing message")
            .description("HTML: grows from two to five rows, then scrolls.")
            .control(|field|sapid::textarea().field(field).autosize(true).min_rows(2).max_rows(5)
                .placeholder("Type several lines or resize the page to see wrapping change the height.")),
        sapid::text().theme(theme).content("Uses FieldSizing::Content on supported HTML hosts. Native Apace explicitly rejects autosize; use fixed rows there."),
    ))
}
