use glory::prelude::*;
use sapid::charts::*;
fn points() -> PlotData {
    PlotData::Points(vec![
        PointDatum {
            key: "north".into(),
            label: "North".into(),
            x: 1.,
            y: 3.,
            size: 1.,
        },
        PointDatum {
            key: "south".into(),
            label: "South".into(),
            x: 4.,
            y: 2.,
            size: 4.,
        },
        PointDatum {
            key: "west".into(),
            label: "West".into(),
            x: 2.,
            y: 5.,
            size: 9.,
        },
        PointDatum {
            key: "east".into(),
            label: "East".into(),
            x: 5.,
            y: 4.,
            size: 3.,
        },
    ])
}
fn sample_data() -> PlotData {
    points()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        bubble_chart("bubble-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
            .default_options(PlotOptions {
                bubble_radius: Some(BubbleRadius { min: 3., max: 32. }),
                ..Default::default()
            })
    }
}
