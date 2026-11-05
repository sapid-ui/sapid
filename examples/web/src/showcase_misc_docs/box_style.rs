use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::box_()
        .theme(theme.clone())
        .style(Style {
            background: Some(theme.surface),
            border_color: Some(theme.border),
            ..Style::new()
                .padding(Edges::all(24.))
                .radius(12.)
                .border_width(Edges::all(1.))
                .border_style(glory::style::BorderStyle::Solid)
        })
        .fill(
            sapid::text()
                .theme(theme.clone())
                .content("A quiet bordered container with room for its content."),
        )
}
