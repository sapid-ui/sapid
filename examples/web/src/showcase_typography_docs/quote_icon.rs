use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::blockquote()
        .theme(theme.clone())
        .fill(
            sapid::group()
                .theme(theme.clone())
                .align(Align::Start)
                .wrap(glory::style::FlexWrap::NoWrap)
                .gap(12.)
                .fill((
                    sapid::theme_icon(sapid::Icon::Info)
                        .theme(theme.clone())
                        .size(sapid::Size::Small),
                    sapid::text()
                        .theme(theme.clone())
                        .style(Style {
                            grow: Some(1.),
                            ..Style::new().min_width(Length::Px(0.))
                        })
                        .content(TEXT_1),
                )),
        )
        .citation(
            sapid::text()
                .theme(theme.clone())
                .variant(sapid::TextVariant::Small)
                .content("— Team working agreement"),
        )
}

const TEXT_1: &str =
    "Make the next step clear, and let people decide when they are ready to take it.";
