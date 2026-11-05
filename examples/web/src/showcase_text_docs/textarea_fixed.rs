use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("textarea-fixed")
            .theme(theme.clone())
            .label("Fixed three rows")
            .description("More content scrolls inside this editor.")
            .control(|field| {
                sapid::textarea().field(field).rows(3).default_value(
                    "First line\nSecond line\nThird line\nFourth line\nFifth line\nSixth line",
                )
            }),
        sapid::field("textarea-limits")
            .theme(theme)
            .label("Four rows within two to five")
            .control(|field| {
                sapid::textarea()
                    .field(field)
                    .rows(4)
                    .min_rows(2)
                    .max_rows(5)
                    .placeholder("The fixed height and its limits are separate declarations.")
            }),
    ))
}
