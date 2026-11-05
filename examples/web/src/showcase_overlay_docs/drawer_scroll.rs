use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::drawer("drawer-scroll")
        .theme(theme)
        .label("Open activity drawer")
        .title("Workspace activity")
        .content(move || {
            view()
                .class("overlay-doc-scroll")
                .style(
                    Style::new()
                        .height(Length::Px(260.))
                        .overflow_y(Overflow::Auto),
                )
                .fill(
                    sapid::stack().theme(body.clone()).gap(16.).fill(
                        (1..=18)
                            .map(|i| {
                                sapid::text().theme(body.clone()).content(format!(
                                    "Activity {i}: A team member updated a shared project document."
                                ))
                            })
                            .collect::<Vec<_>>(),
                    ),
                )
        })
}
