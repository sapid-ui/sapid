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
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        matrix_chart("matrix-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
    }
}
