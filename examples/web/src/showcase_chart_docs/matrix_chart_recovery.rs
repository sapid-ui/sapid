use glory::prelude::*;
use sapid::charts::*;
fn matrix() -> PlotData {
    PlotData::Matrix(MatrixData {
        rows: vec![
            ChartCategory::new("product-a", "Product A"),
            ChartCategory::new("product-b", "Product B"),
            ChartCategory::new("product-c", "Product C"),
        ],
        columns: vec![
            ChartCategory::new("q1", "Q1"),
            ChartCategory::new("q2", "Q2"),
            ChartCategory::new("q3", "Q3"),
        ],
        cells: [
            ("a1", "product-a", "q1", 4.),
            ("a2", "product-a", "q2", 7.),
            ("a3", "product-a", "q3", 10.),
            ("b1", "product-b", "q1", 2.),
            ("b2", "product-b", "q2", -3.),
            ("b3", "product-b", "q3", 6.),
            ("c1", "product-c", "q1", 8.),
            ("c2", "product-c", "q2", 5.),
            ("c3", "product-c", "q3", 1.),
        ]
        .into_iter()
        .map(|(key, row, column, value)| MatrixCell {
            key: key.into(),
            row: row.into(),
            column: column.into(),
            value,
        })
        .collect(),
    })
}
fn sample_data() -> PlotData {
    matrix()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(0u8);
        let feedback = scope.cage("No chart validation errors.".to_owned());
        let data = scope.bond(move || match state.get() {
            1 => PlotData::Matrix(MatrixData::default()),
            2 => PlotData::Values(vec![ValueDatum::new("invalid", "Rejected value", f64::NAN)]),
            _ => sample_data(),
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Show empty data")
                        .on_press(move || state.revise(|old| *old = 1)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Reject invalid update")
                        .on_press(move || state.revise(|old| *old = 2)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Restore chart data")
                        .on_press(move || state.revise(|old| *old = 0)),
                )),
                sapid::text().theme(theme.clone()).content(feedback),
                matrix_chart("matrix-chart-recovery")
                    .theme(theme.clone())
                    .data(data)
                    .on_error(move |error| {
                        feedback.revise(|v| *v = format!("Rejected update: {error}"))
                    }),
            ))
            .show_in(scope)
    })
}
