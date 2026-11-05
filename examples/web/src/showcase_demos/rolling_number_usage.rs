use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(128.);
        let animated = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content("Active projects"),
                sapid::rolling_number()
                    .theme(theme.clone())
                    .value(value)
                    .duration_millis(500u16)
                    .animated(animated)
                    .style(Style {
                        font_size: Some(38.),
                        ..Style::new()
                    }),
                sapid::group().theme(theme.clone()).gap(10.).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Add one project")
                        .on_press(move || value.revise(|v| *v += 1.)),
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Outline)
                        .label("Remove one project")
                        .on_press(move || value.revise(|v| *v -= 1.)),
                )),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate project count")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|s| *s = v)),
            ))
            .show_in(scope)
    })
}
