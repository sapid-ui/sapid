use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        sapid::button()
            .theme(theme.clone())
            .unstyled()
            .label(count.map(|v| format!("Open workspace ({v})")))
            .on_press(move || count.revise(|v| *v += 1))
            .show_in(scope)
    })
}
