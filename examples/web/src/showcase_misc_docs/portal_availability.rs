use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::portal_context(scope, "available-project-note")?;
        let available = scope.cage(true);
        let status = scope.cage("Your note is available in the panel.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show project note destination")
                    .checked(available)
                    .on_checked_change(move |v| available.revise(|s| *s = v)),
                when(available, {
                    let context = context.clone();
                    let theme = theme.clone();
                    move || {
                        sapid::paper().theme(theme.clone()).bordered(true).fill(
                            sapid::portal_target(&context, "note").fill(
                                sapid::title(4)
                                    .theme(theme.clone())
                                    .weight(400.)
                                    .content("Project note destination"),
                            ),
                        )
                    }
                }),
                sapid::portal(&context, "note")
                    .on_target_lost(move |_| {
                        status.revise(|s| {
                            *s = "The note is kept until its destination returns.".to_owned()
                        })
                    })
                    .content({
                        let theme = theme.clone();
                        move || {
                            sapid::field("parked-project-note")
                                .theme(theme.clone())
                                .label("Parked project note")
                                .control(|field| {
                                    sapid::text_field()
                                        .field(field)
                                        .default_value("Keep this draft.")
                                })
                        }
                    }),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
