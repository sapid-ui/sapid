use glory::prelude::*;
use sapid::charts::*;
fn values(values: &[(&str, &str, f64)]) -> PlotData {
    PlotData::Values(
        values
            .iter()
            .map(|(key, label, value)| ValueDatum::new(*key, *label, *value))
            .collect(),
    )
}
fn sample_data() -> PlotData {
    values(&[
        ("w1", "Week 1", 4.0),
        ("w2", "Week 2", 7.0),
        ("w3", "Week 3", 5.0),
        ("w4", "Week 4", 9.0),
        ("w5", "Week 5", 8.0),
        ("w6", "Week 6", 12.0),
        ("w7", "Week 7", 15.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).fill((
        sapid::text()
            .theme(theme.clone())
            .content("Active subscriptions: 15 · seven-week trend"),
        sparkline("sparkline-overview")
            .theme(theme.clone())
            .data(sample_data())
            .style(
                Style::new()
                    .width(Length::Px(320.))
                    .max_width(Limit::Percent(100.)),
            )
            .legend(false)
            .navigation(false)
            .data_table(false)
            .frame(ChartFrame {
                width: 640.,
                height: 160.,
            }),
        sapid::text()
            .theme(theme.clone())
            .content("Weekly values: 4, 7, 5, 9, 8, 12, 15"),
    ))
}
