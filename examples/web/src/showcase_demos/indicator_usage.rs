use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let opened = scope.cage(0u32);
        sapid::stack()
            .align(Align::Start)
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::indicator()
                    .theme(theme.clone())
                    .diameter(22.)
                    .offset(2.)
                    .label("3")
                    .name("3 unread notifications")
                    .visible(visible)
                    .fill(
                        sapid::button()
                            .theme(theme.clone())
                            .variant(sapid::ButtonVariant::Outline)
                            .label("Open inbox")
                            .on_press(move || opened.revise(|v| *v += 1)),
                    ),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show unread count")
                    .checked(visible)
                    .on_checked_change(move |v| visible.revise(|s| *s = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(opened.map(|n| format!("Inbox opened {n} times"))),
            ))
            .show_in(scope)
    })
}
