use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::responsive_root(640.).content(move |context| {
        widget({
            let theme = theme.clone();
            move |scope| {
                let column = Style::new().flex_direction(FlexDirection::Column).gap(12.);
                let row = Style {
                    align: Some(Align::Center),
                    justify: Some(Justify::Between),
                    ..Style::new().flex_direction(FlexDirection::Row).gap(24.)
                };
                let style = context.styles(sapid::ResponsiveValues::new(
                    column.clone(),
                    row.clone(),
                    row.clone(),
                    row.clone(),
                    row,
                ))?;
                sapid::flex()
                    .theme(theme.clone())
                    .class("docs-flex-responsive")
                    .style(style)
                    .fill((
                        sapid::text_field()
                            .theme(theme.clone())
                            .name("Responsive project name")
                            .placeholder("Project name")
                            .style(
                                Style::new()
                                    .min_width(Length::Px(0.))
                                    .width(Length::Percent(100.)),
                            ),
                        sapid::button().theme(theme.clone()).label("Create project"),
                    ))
                    .show_in(scope)
            }
        })
    })
}
