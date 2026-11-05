use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [
            sapid::NumberLocale::EnUs,
            sapid::NumberLocale::DeDe,
            sapid::NumberLocale::ZhCn,
        ]
        .into_iter()
        .map(|locale| {
            sapid::group().theme(theme.clone()).gap(12.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(format!("{locale:?}")),
                sapid::number_formatter()
                    .theme(theme.clone())
                    .locale(locale)
                    .value(-1234567.89)
                    .min_fraction_digits(2)
                    .max_fraction_digits(2),
            ))
        })
        .collect::<Vec<_>>(),
    )
}
