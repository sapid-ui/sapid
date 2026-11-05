use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Filled button")
                    .on_press(move || count.revise(|v| *v += 1)),
                sapid::text()
                    .theme(theme.clone())
                    .content(count.map(|v| format!("Button presses: {v}"))),
            ))
            .show_in(scope)
    })
}
