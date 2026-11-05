use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let metric = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(-1250.5)
                    .prefix("Balance: ")
                    .suffix(" credits")
                    .min_fraction_digits(2)
                    .max_fraction_digits(2),
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(34.5)
                    .format(sapid::NumberFormat::Currency(sapid::Currency::Eur))
                    .locale(sapid::NumberLocale::DeDe),
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(12.75)
                    .suffix(metric.map(|v| {
                        if *v {
                            " km".to_owned()
                        } else {
                            " mi".to_owned()
                        }
                    })),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use kilometer suffix")
                    .checked(metric)
                    .on_checked_change(move |v| metric.revise(|s| *s = v)),
            ))
            .show_in(scope)
    })
}
