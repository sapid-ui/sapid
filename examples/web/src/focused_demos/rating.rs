use glory::prelude::*;
use sapid::{RatingPrecision, Theme};

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    assert_eq!(id, "P7-05");
    Box::new(widget(move |scope| {
        let value = scope.cage(2.0);
        let color = scope.cage("Yellow".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let count = scope.cage(Some("5".to_owned()));
        let precision = scope.cage(Some("whole".to_owned()));
        let highlight = scope.cage(false);
        let hover = scope.cage(true);
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        let enabled = scope.bond(move || !disabled.get());
        let rating_theme = scope.bond({
            let theme = theme.clone();
            move || super::controls::accent(theme.clone(), &color.get())
        });
        let revision = scope.bond(move || {
            vec![format!(
                "{}-{}-{}-{}",
                color.get(),
                size.get().unwrap_or_default(),
                count.get().unwrap_or_default(),
                precision.get().unwrap_or_default()
            )]
        });
        let preview = each(revision)
            .key(|revision| revision.clone())
            .children(move |entry| {
                let size = super::controls::size_value(&entry.value.get());
                let count = count
                    .get()
                    .and_then(|value| value.parse::<u8>().ok())
                    .unwrap_or(5);
                let precision = if precision.get().as_deref() == Some("half") {
                    RatingPrecision::Half
                } else {
                    RatingPrecision::Whole
                };
                sapid::rating("playground-rating")
                    .name("Product rating")
                    .theme(rating_theme.get())
                    .size(size)
                    .count(count)
                    .precision(precision)
                    .value(value)
                    .highlight_selected_only(highlight)
                    .hover_preview(hover)
                    .readonly(readonly)
                    .enabled(enabled)
                    .on_value_change(move |next| value.revise(|current| *current = next))
            });
        let controls = vec![
            super::controls::color_choices(color, &theme),
            super::controls::size(size, &theme),
            super::controls::select(
                "count",
                "Count",
                count,
                &[
                    ("3", "3"),
                    ("4", "4"),
                    ("5", "5"),
                    ("6", "6"),
                    ("7", "7"),
                    ("8", "8"),
                ],
                &theme,
            ),
            super::controls::select(
                "precision",
                "Precision",
                precision,
                &[("whole", "Whole stars"), ("half", "Half stars")],
                &theme,
            ),
            super::controls::checkbox("Highlight selected only", highlight, &theme),
            super::controls::checkbox("Preview on hover", hover, &theme),
            super::controls::checkbox("Read only", readonly, &theme),
            super::controls::checkbox("Disabled", disabled, &theme),
        ];
        super::controls::playground(Box::new(preview), controls).show_in(scope)
    }))
}
