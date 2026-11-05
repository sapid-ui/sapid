use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let saved = scope.cage(false);
        sapid::card()
            .theme(theme.clone())
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(glory::style::Limit::Px(420.)),
            )
            .header(
                sapid::image(media("alpine-lake-photo.png"))
                    .theme(theme.clone())
                    .alt("An alpine village beside a turquoise lake")
                    .fit(glory::style::ObjectFit::Cover)
                    .width(Length::Percent(100.))
                    .height(Length::Px(190.))
                    .style(Style::new().radius(8.)),
            )
            .title("A lakeside weekend")
            .description("A quiet mountain escape with room to explore.")
            .fill(
                sapid::stack().theme(theme.clone()).gap(12.).fill((
                    sapid::badge()
                        .theme(theme.clone())
                        .variant(sapid::BadgeVariant::Dot)
                        .content("3 nights · From $240"),
                    sapid::text().theme(theme.clone()).content(TEXT_1),
                )),
            )
            .footer(
                sapid::stack().theme(theme.clone()).gap(10.).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label(saved.map(|s| {
                            if *s {
                                "Trip idea saved".to_owned()
                            } else {
                                "Save trip idea".to_owned()
                            }
                        }))
                        .on_press(move || saved.revise(|s| *s = !*s)),
                    sapid::text().theme(theme.clone()).content(saved.map(|s| {
                        if *s {
                            "Added to your local trip ideas.".to_owned()
                        } else {
                            "Save this idea for later.".to_owned()
                        }
                    })),
                )),
            )
            .show_in(scope)
    })
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}

const TEXT_1: &str = "Walk the shoreline, visit the village and discover a new trail each morning.";
