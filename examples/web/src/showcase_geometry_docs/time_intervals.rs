use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(
        [
            ("Quarter-hour slots", 900, 3),
            ("Dense column window", 1800, 12),
        ]
        .into_iter()
        .map(|(name, step, cols)| {
            sapid::time_grid(name.replace(' ', "-"))
                .theme(theme.clone())
                .name(name)
                .spec(
                    sapid::TimeGridSpec::new(
                        sapid::LocalTime::new(9, 0, 0).unwrap(),
                        sapid::LocalTime::new(if cols == 12 { 15 } else { 12 }, 0, 0).unwrap(),
                        step,
                        cols,
                    )
                    .unwrap(),
                )
                .format(sapid::TimeFormat::new(
                    sapid::HourCycle::H12,
                    sapid::TimePrecision::Minute,
                    sapid::DateLocale::En,
                ))
        })
        .collect::<Vec<_>>(),
    )
}
