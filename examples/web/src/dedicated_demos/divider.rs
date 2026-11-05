use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let chosen = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content("Sign in with your workspace account."),
                sapid::separator()
                    .theme(theme.clone())
                    .label("or continue with email"),
                sapid::button()
                    .theme(theme.clone())
                    .variant(sapid::ButtonVariant::Outline)
                    .label("Continue with email")
                    .on_press(move || chosen.revise(|s| *s = true)),
                sapid::text().theme(theme.clone()).content(chosen.map(|v| {
                    if *v {
                        "Email sign-in selected.".to_owned()
                    } else {
                        "Choose the sign-in method that works for you.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
