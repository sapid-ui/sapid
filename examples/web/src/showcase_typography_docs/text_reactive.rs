use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ready = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("text-doc-retained")
                    .content(ready.map(|v| {
                        if *v {
                            "Your project is ready to share.".to_owned()
                        } else {
                            "Your project is still a draft.".to_owned()
                        }
                    })),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Project is ready")
                    .checked(ready)
                    .on_checked_change(move |v| ready.revise(|s| *s = v)),
                sapid::text()
                    .unstyled()
                    .style(Style {
                        color: Some(theme.foreground),
                        font_size: Some(16.),
                        ..Style::new()
                    })
                    .content("Explicit text appearance"),
            ))
            .show_in(scope)
    })
}
