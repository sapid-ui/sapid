use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show loading indicators")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::field("password-loading-right")
                    .theme(theme.clone())
                    .label("Loading on the right")
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .loading(loading)
                            .default_value("Right loading sample")
                    }),
                sapid::field("password-loading-left")
                    .theme(theme.clone())
                    .label("Loading on the left")
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .loading(loading)
                            .loading_position(sapid::InputSectionPosition::Left)
                            .default_value("Left loading sample")
                    }),
            ))
            .show_in(scope)
    })
}
