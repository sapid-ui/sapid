use glory::prelude::*;
use sapid::charts::*;
fn hierarchy() -> PlotData {
    PlotData::Hierarchy(vec![
        HierarchyDatum {
            key: "regions".into(),
            label: "Regions".into(),
            parent: None,
            weight: None,
        },
        HierarchyDatum {
            key: "north".into(),
            label: "North".into(),
            parent: Some("regions".into()),
            weight: Some(3.),
        },
        HierarchyDatum {
            key: "south".into(),
            label: "South".into(),
            parent: Some("regions".into()),
            weight: Some(7.),
        },
        HierarchyDatum {
            key: "west".into(),
            label: "West".into(),
            parent: Some("regions".into()),
            weight: Some(5.),
        },
    ])
}
fn sample_data() -> PlotData {
    hierarchy()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        sunburst_chart("sunburst-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
    }
}
