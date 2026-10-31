use glory::prelude::*;
use sapid::Theme;

fn progress_preview(
    theme: Theme,
    current: Vary<f64>,
    value_text: Vary<String>,
    line_width: Vary<f64>,
    thickness_value: Vary<f64>,
    indeterminate: Vary<bool>,
) -> impl Widget {
    let ring_theme = theme.clone();
    let semi_theme = theme.clone();
    sapid::stack().theme(theme.clone()).gap(20.0).fill((
        sapid::progress(current.clone())
            .theme(theme.clone())
            .name("File upload")
            .value_text(value_text.clone())
            .indeterminate(indeterminate.clone())
            .size(line_width.get())
            .thickness(thickness_value.get()),
        sapid::group().theme(theme).gap(24.0).fill((
            sapid::ring_progress(current.clone())
                .theme(ring_theme)
                .name("Circular upload")
                .value_text(value_text.clone())
                .indeterminate(indeterminate.clone())
                .size(100.0)
                .thickness(thickness_value.get()),
            sapid::semi_circle_progress(current)
                .theme(semi_theme)
                .name("Half-circle upload")
                .value_text(value_text)
                .indeterminate(indeterminate)
                .size(120.0)
                .thickness(thickness_value.get()),
        )),
    ))
}

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    match id {
        "P7-02" => Box::new(widget(move |scope| {
            let value = scope.cage(40.0);
            let color = scope.cage("Blue".to_owned());
            let size = scope.cage(Some("md".to_owned()));
            let step = scope.cage(Some("5".to_owned()));
            let readonly = scope.cage(false);
            let disabled = scope.cage(false);
            let enabled = scope.bond(move || !disabled.get());
            let slider_theme = scope.bond({
                let theme = theme.clone();
                move || super::controls::accent(theme.clone(), &color.get())
            });
            let revision = scope.bond(move || {
                vec![format!(
                    "{}-{}",
                    color.get(),
                    size.get().unwrap_or_default()
                )]
            });
            let preview = each(revision)
                .key(|revision| revision.clone())
                .children(move |entry| {
                    let size = super::controls::size_value(&entry.value.get());
                    let step = step
                        .get()
                        .clone()
                        .and_then(|value| value.parse::<f64>().ok())
                        .unwrap_or(5.0);
                    let value_text = value.map(|value| Some(format!("{value} units")));
                    sapid::slider("playground-slider")
                        .name("Volume")
                        .theme(slider_theme.get())
                        .size(size)
                        .min(0.0)
                        .max(100.0)
                        .step(step)
                        .value(value)
                        .value_text(value_text)
                        .readonly(readonly)
                        .enabled(enabled)
                        .style(
                            Style::new()
                                .width(Length::Percent(100.0))
                                .max_width(Limit::Px(420.0)),
                        )
                        .on_value_change(move |next| value.revise(|current| *current = next))
                });
            let controls = vec![
                super::controls::color_choices(color, &theme),
                super::controls::size(size, &theme),
                super::controls::select(
                    "step",
                    "Step",
                    step,
                    &[("1", "1"), ("5", "5"), ("10", "10"), ("25", "25")],
                    &theme,
                ),
                super::controls::checkbox("Read only", readonly, &theme),
                super::controls::checkbox("Disabled", disabled, &theme),
            ];
            super::controls::playground(Box::new(preview), controls).show_in(scope)
        })),
        "P5-08" => Box::new(widget(move |scope| {
            let value = scope.cage(Some("40".to_owned()));
            let color = scope.cage("Blue".to_owned());
            let width = scope.cage(Some("300".to_owned()));
            let thickness = scope.cage(Some("8".to_owned()));
            let indeterminate = scope.cage(false);
            let progress_theme = scope.bond({
                let theme = theme.clone();
                move || super::controls::accent(theme.clone(), &color.get())
            });
            let current = value.map(|value| {
                value
                    .clone()
                    .and_then(|value| value.parse::<f64>().ok())
                    .unwrap_or(40.0)
            });
            let value_text = scope.bond(move || format!("{}% complete", current.get()));
            let line_width = width.map(|value| {
                value
                    .clone()
                    .and_then(|value| value.parse::<f64>().ok())
                    .unwrap_or(300.0)
            });
            let thickness_value = thickness.map(|value| {
                value
                    .clone()
                    .and_then(|value| value.parse::<f64>().ok())
                    .unwrap_or(8.0)
            });
            let preview = each(scope.bond(move || vec![color.get()]))
                .key(|color| color.clone())
                .children(move |_| {
                    progress_preview(
                        progress_theme.get(),
                        current.into(),
                        value_text.into(),
                        line_width.into(),
                        thickness_value.into(),
                        indeterminate.into(),
                    )
                });
            let controls = vec![
                super::controls::color_choices(color, &theme),
                super::controls::select(
                    "value",
                    "Value",
                    value,
                    &[
                        ("0", "0%"),
                        ("25", "25%"),
                        ("40", "40%"),
                        ("75", "75%"),
                        ("100", "100%"),
                    ],
                    &theme,
                ),
                super::controls::select(
                    "width",
                    "Line width",
                    width,
                    &[("200", "200 px"), ("300", "300 px"), ("400", "400 px")],
                    &theme,
                ),
                super::controls::select(
                    "thickness",
                    "Thickness",
                    thickness,
                    &[("4", "4 px"), ("8", "8 px"), ("12", "12 px")],
                    &theme,
                ),
                super::controls::checkbox("Indeterminate", indeterminate, &theme),
            ];
            super::controls::playground(Box::new(preview), controls).show_in(scope)
        })),
        _ => unreachable!("unsupported measurement demo"),
    }
}
