use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(99.);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::rolling_number()
                    .theme(theme.clone())
                    .value(value)
                    .duration_millis(600u16),
                sapid::group().theme(theme.clone()).gap(8.).fill(
                    [99., 100., 999., 1000., -12.]
                        .into_iter()
                        .map(|next| {
                            sapid::button()
                                .theme(theme.clone())
                                .variant(sapid::ButtonVariant::Outline)
                                .label(format!("Set {next}"))
                                .on_press(move || value.revise(|v| *v = next))
                        })
                        .collect::<Vec<_>>(),
                ),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Current count: {v}"))),
            ))
            .show_in(scope)
    })
}
