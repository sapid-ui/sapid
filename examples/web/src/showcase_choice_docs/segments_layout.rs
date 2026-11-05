use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .gap(20.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(360.)),
        )
        .fill((
            sapid::segmented_control("period-vertical")
                .theme(theme.clone())
                .label("Vertical period")
                .orientation(glory::render::Orientation::Vertical)
                .full_width(true)
                .default_value("day")
                .segments([
                    sapid::segment("day").label("Daily"),
                    sapid::segment("week").label("Weekly"),
                    sapid::segment("month").label("Monthly").enabled(false),
                ]),
            sapid::segmented_control("long-segments")
                .theme(theme)
                .label("Workspace section")
                .full_width(true)
                .size(sapid::Size::Small)
                .default_value("team")
                .segments([
                    sapid::segment("team").label("Team settings"),
                    sapid::segment("reports").label("Project reports"),
                    sapid::segment("access").label("Access control"),
                ]),
        ))
}
