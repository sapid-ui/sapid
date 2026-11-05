use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let code = scope.cage(String::new());
        let status = scope.cage("Enter the six-digit code".to_owned());
        let input_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("signin-code")
                    .theme(theme.clone())
                    .label("Verification code")
                    .description("Use the code from your authenticator")
                    .control(move |field| {
                        sapid::input_otp()
                            .theme(input_theme.clone())
                            .field(field)
                            .value(code)
                            .on_value_change(move |next| {
                                code.revise(|old| *old = next);
                                status.revise(|old| *old = "Enter the six-digit code".to_owned());
                            })
                            .on_complete(move |_| {
                                status.revise(|old| *old = "Code ready to verify".to_owned())
                            })
                            .on_reject(move |_| {
                                status.revise(|old| {
                                    *old = "Use six ASCII digits without separators".to_owned()
                                })
                            })
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || status.get())),
            ))
            .show_in(scope)
    })
}
