use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::title(3)
            .theme(theme.clone())
            .class("title-doc-truncate")
            .weight(400.)
            .truncate(true)
            .style(
                Style::new()
                    .width(Length::Px(240.))
                    .max_width(glory::style::Limit::Percent(100.)),
            )
            .content("A long project heading with all the details retained"),
        sapid::title(3)
            .theme(theme.clone())
            .class("title-doc-clamp")
            .weight(400.)
            .line_clamp(2)
            .style(
                Style::new()
                    .width(Length::Px(260.))
                    .max_width(glory::style::Limit::Percent(100.)),
            )
            .content(TEXT_1),
    ))
}

const TEXT_1: &str = "A heading can be constrained to two lines while retaining its complete semantic name and heading level.";
