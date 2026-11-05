use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let pressed = scope.cage(false);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::toggle()
                    .theme(theme.clone())
                    .label("Bold selection")
                    .pressed(pressed)
                    .on_pressed_change(move |v| {
                        if accept.get() {
                            pressed.revise(|old| *old = v)
                        }
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept pressed proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(pressed.map(|v| format!("Bold enabled: {v}"))),
            ))
            .show_in(scope)
    })
}
