use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let result = scope.cage("No action chosen".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::action_bar("rtl-actions")
                    .theme(theme.clone())
                    .selected_count(2usize)
                    .direction(glory::render::Direction::Rtl)
                    .group(
                        sapid::toolbar_group("file-actions")
                            .button(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Move selected")
                                    .on_press(move || {
                                        result.revise(|v| *v = "Move chosen".to_owned())
                                    }),
                            )
                            .button(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Unavailable download")
                                    .enabled(false),
                            ),
                    )
                    .group(
                        sapid::toolbar_group("sharing-actions").button(
                            sapid::button()
                                .theme(theme.clone())
                                .label("Share selected")
                                .on_press(move || {
                                    result.revise(|v| *v = "Share chosen".to_owned())
                                }),
                        ),
                    ),
                sapid::text().theme(theme.clone()).content(result),
            ))
            .show_in(scope)
    })
}
