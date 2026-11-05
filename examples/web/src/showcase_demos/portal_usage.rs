use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::portal_context(scope, "primary-project-note")?;
        let target = scope.cage("board".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::group().theme(theme.clone()).gap(16.).fill(
                    [("board", "Project board"), ("sidebar", "Personal notes")]
                        .into_iter()
                        .map(|(key, title)| {
                            sapid::paper()
                                .theme(theme.clone())
                                .bordered(true)
                                .style(Style {
                                    grow: Some(1.),
                                    ..Style::new()
                                        .min_width(Length::Px(220.))
                                        .padding(Edges::all(16.))
                                })
                                .fill(
                                    sapid::portal_target(&context, key).fill(
                                        sapid::title(4)
                                            .theme(theme.clone())
                                            .weight(400.)
                                            .content(title),
                                    ),
                                )
                        })
                        .collect::<Vec<_>>(),
                ),
                sapid::portal(&context, target).content({
                    let theme = theme.clone();
                    move || {
                        sapid::field("portal-primary-note")
                            .theme(theme.clone())
                            .label("Movable project note")
                            .control(|field| {
                                sapid::text_field()
                                    .field(field)
                                    .default_value("Review the latest prototype.")
                            })
                    }
                }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Move project note")
                    .on_press(move || {
                        target.revise(|s| {
                            *s = if s == "board" {
                                "sidebar".to_owned()
                            } else {
                                "board".to_owned()
                            }
                        })
                    }),
            ))
            .show_in(scope)
    })
}
