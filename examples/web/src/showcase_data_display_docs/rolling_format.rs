use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let amount = scope.cage(1250.5);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::rolling_number()
                    .theme(theme.clone())
                    .value(amount)
                    .locale(sapid::NumberLocale::DeDe)
                    .format(sapid::NumberFormat::Currency(sapid::Currency::Eur))
                    .min_fraction_digits(2)
                    .max_fraction_digits(2),
                sapid::rolling_number()
                    .theme(theme.clone())
                    .value(amount.map(|v| *v / 10000.))
                    .format(sapid::NumberFormat::Percent)
                    .max_fraction_digits(1),
                sapid::button()
                    .theme(theme.clone())
                    .label("Increase formatted amount")
                    .on_press(move || amount.revise(|v| *v += 12.5)),
            ))
            .show_in(scope)
    })
}
