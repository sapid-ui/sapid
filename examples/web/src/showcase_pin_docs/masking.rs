use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let masked = scope.cage(true);
        let alpha_theme = theme.clone();
        let protected_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::field("alpha-code")
                    .theme(theme.clone())
                    .label("Alphanumeric code")
                    .control(move |field| {
                        sapid::pin_input()
                            .theme(alpha_theme.clone())
                            .field(field)
                            .length(4)
                            .alphabet(sapid::PinAlphabet::AsciiAlphanumeric)
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Hide code")
                    .checked(masked)
                    .on_checked_change(move |value| masked.revise(|old| *old = value)),
                sapid::field("protected-code")
                    .theme(theme.clone())
                    .label("Protected verification code")
                    .control(move |field| {
                        sapid::input_otp()
                            .theme(protected_theme.clone())
                            .field(field)
                            .masked(masked)
                    }),
            ))
            .show_in(scope)
    })
}
