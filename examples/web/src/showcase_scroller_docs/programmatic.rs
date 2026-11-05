use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = scope.cage(None::<sapid::ScrollHandle>);
        let content_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::group().theme(theme.clone()).fill(
                    [("Start", 0.), ("End", 1.)]
                        .into_iter()
                        .map(|(label, fraction)| {
                            let weak = scope.downgrade();
                            sapid::button()
                                .theme(theme.clone())
                                .label(label)
                                .on_press(move || {
                                    if let Some(handle) = handle.get()
                                        && let Err(error) = handle.scroll_to_fraction(fraction, 0.)
                                        && let Some(scope) = weak.upgrade()
                                    {
                                        scope.report_error(error);
                                    }
                                })
                        })
                        .collect::<Vec<_>>(),
                ),
                sapid::scroller("programmatic-strip")
                    .controls(false)
                    .area(|area| {
                        area.theme(theme.clone())
                            .name("Draft workspace")
                            .style(Style::new().height(Length::Px(96.)))
                            .on_ready(move |value| handle.revise(|old| *old = Some(value)))
                    })
                    .content(move |_| {
                        sapid::group()
                            .theme(content_theme.clone())
                            .wrap(glory::style::FlexWrap::NoWrap)
                            .gap(16.)
                            .fill((
                                sapid::text_field()
                                    .theme(content_theme.clone())
                                    .name("Retained draft")
                                    .placeholder("Type a draft")
                                    .style(
                                        Style::new()
                                            .width(Length::Px(220.))
                                            .min_width(Length::Px(220.)),
                                    ),
                                glory::ui::view()
                                    .style(
                                        Style::new()
                                            .width(Length::Px(900.))
                                            .min_width(Length::Px(900.)),
                                    )
                                    .fill(text().content("End of the workspace")),
                            ))
                    }),
            ))
            .show_in(scope)
    })
}
