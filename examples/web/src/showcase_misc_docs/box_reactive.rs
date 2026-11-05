use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let emphasized = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Emphasize this note")
                    .checked(emphasized)
                    .on_checked_change(move |v| emphasized.revise(|s| *s = v)),
                sapid::box_()
                    .theme(theme.clone())
                    .class("misc-reactive-box")
                    .style(emphasized.map({
                        let surface = theme.surface;
                        let accent = theme.accent_soft;
                        move |v| Style {
                            background: Some(if *v { accent } else { surface }),
                            ..Style::new().padding(Edges::all(16.)).radius(8.)
                        }
                    }))
                    .fill(sapid::text().theme(theme.clone()).content(TEXT_1)),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "The note keeps the same widget and content while its surface changes.";
