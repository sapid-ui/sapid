use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let amount = scope.cage(12500.5);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::text().theme(theme.clone()).content("Project budget"),
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(amount)
                    .format(sapid::NumberFormat::Currency(sapid::Currency::Usd))
                    .min_fraction_digits(2)
                    .max_fraction_digits(2)
                    .style(Style {
                        font_size: Some(30.),
                        ..Style::new()
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Add 250 to budget")
                    .on_press(move || amount.revise(|v| *v += 250.)),
            ))
            .show_in(scope)
    })
}
