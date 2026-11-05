use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("text-default")
            .theme(theme.clone())
            .label("Default presentation")
            .control(|field| {
                sapid::text_field()
                    .field(field)
                    .placeholder("Standard bordered input")
            }),
        sapid::field("text-filled")
            .theme(theme.clone())
            .label("Filled presentation")
            .control({
                let theme = theme.clone();
                move |field| {
                    sapid::text_field()
                        .field(field)
                        .style(Style {
                            background: Some(theme.accent_soft),
                            border_color: Some(theme.accent_soft),
                            ..Style::new()
                        })
                        .placeholder("Explicit filled style")
                }
            }),
        sapid::field("text-borderless")
            .theme(theme)
            .label("Borderless presentation")
            .control(|field| {
                sapid::text_field()
                    .field(field)
                    .style(Style::new().border_width(Edges::all(0.)))
                    .placeholder("Same editor without a border")
            }),
    ))
}
