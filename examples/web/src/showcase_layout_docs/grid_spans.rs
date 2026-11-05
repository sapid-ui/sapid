use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let card = |label| {
        sapid::card()
            .theme(theme.clone())
            .fill(sapid::text().theme(theme.clone()).content(label))
    };
    sapid::grid("docs-grid-spans").theme(theme.clone()).columns(6).gap(12.).class("docs-grid-spans")
        .item(sapid::grid_item("article").column(1).row(1).span(4).row_span(2).fill(card("A longer article occupies four columns and two rows. Other cells retain their own content and fill the remaining two columns.")))
        .item(sapid::grid_item("summary").column(5).row(1).span(2).fill(card("Summary")))
        .item(sapid::grid_item("actions").column(5).row(2).span(2).fill(card("Actions")))
        .item(sapid::grid_item("offset").column(3).row(3).span(4).fill(card("Starts at column 3: two columns remain empty before this item.")))
}
