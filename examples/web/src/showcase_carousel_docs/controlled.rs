use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(0usize);
        let accept = scope.cage(true);
        let inner = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept slide changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::carousel()
                    .theme(theme.clone())
                    .name("Controlled slides")
                    .count(5usize)
                    .current(current)
                    .with_indicators(true)
                    .on_current_change(move |next| {
                        if accept.get() {
                            current.revise(|old| *old = next);
                        }
                    })
                    .render_slide(move |index| {
                        sapid::text()
                            .theme(inner.clone())
                            .content(format!("Chapter {}", index + 1))
                            .style(Style {
                                background: Some(inner.accent_soft),
                                ..Style::new().height(Length::Px(140.)).padding(Edges {
                                    top: 48.,
                                    bottom: 32.,
                                    left: 52.,
                                    right: 52.,
                                })
                            })
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(current.map(|index| format!("Accepted chapter: {}", index + 1))),
            ))
            .show_in(scope)
    })
}
