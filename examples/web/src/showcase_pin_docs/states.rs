use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let error_theme = theme.clone();
    let readonly_theme = theme.clone();
    let disabled_theme = theme.clone();
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::field("expired-code")
            .theme(theme.clone())
            .label("Expired verification code")
            .required(true)
            .error("This code has expired")
            .control(move |field| sapid::input_otp().theme(error_theme.clone()).field(field)),
        sapid::field("readonly-code")
            .theme(theme.clone())
            .label("Read-only code")
            .readonly(true)
            .control(move |field| {
                sapid::input_otp()
                    .theme(readonly_theme.clone())
                    .field(field)
                    .default_value("000001")
            }),
        sapid::field("disabled-code")
            .theme(theme)
            .label("Unavailable code")
            .enabled(false)
            .control(move |field| {
                sapid::input_otp()
                    .theme(disabled_theme.clone())
                    .field(field)
            }),
    ))
}
