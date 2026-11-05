use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = scope.cage(None::<sapid::ScrollHandle>);
        let weak = scope.downgrade();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).gap(10.).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Read notes from the start")
                        .on_press({
                            let weak = weak.clone();
                            move || {
                                if let Some(handle) = handle.get()
 && let Err(error) = handle.scroll_to(0., 0.)
 && let Some(scope) = weak.upgrade() { scope.report_error(error) }
                            }
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Jump to latest notes")
                        .on_press(move || {
                            if let Some(handle) = handle.get()
 && let Err(error) = handle.scroll_to_fraction(0., 1.)
 && let Some(scope) = weak.upgrade() { scope.report_error(error) }
                        }),
                )),
                sapid::scroll_area("positioned-reading")
                    .theme(theme.clone())
                    .name("Positioned reading notes")
                    .on_ready(move |ready| handle.revise(|h| *h = Some(ready)))
                    .style(
                        Style::new()
                            .height(Length::Px(200.))
                            .max_width(glory::style::Limit::Percent(100.)),
                    )
                    .content({
                        let theme = theme.clone();
                        move |_| {
                            sapid::stack().theme(theme.clone()).gap(14.).fill((
                                sapid::title(4)
                                    .theme(theme.clone())
                                    .weight(400.)
                                    .content("Studio reading notes"),
                                sapid::stack().theme(theme.clone()).gap(16.).fill(
                                    (1..=18)
                                        .map(|n| {
                                            sapid::text()
                                                .theme(theme.clone())
                                                .content(format!("Note {n}: Keep project decisions clear, review the details together and make the next action easy to understand."))
                                        })
                                        .collect::<Vec<_>>(),
                                ),
                            ))
                        }
                    }),
            ))
            .show_in(scope)
    })
}
