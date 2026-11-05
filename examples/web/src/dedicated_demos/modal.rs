use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let draft = scope.cage("Ada".to_owned());
        let saved = scope.cage("Ada".to_owned());
        let content_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::dialog("profile-editor")
                    .theme(theme.clone())
                    .label("Edit profile")
                    .title("Edit your profile")
                    .description("Update the name your team sees.")
                    .close_label("Close profile editor")
                    .open(open)
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .content_policy(sapid::ContentPolicy::KeepMounted)
                    .content(move || {
                        sapid::stack().theme(content_theme.clone()).gap(12.).fill((
                            sapid::text_field()
                                .theme(content_theme.clone())
                                .name("Profile display name")
                                .value(draft)
                                .on_value_change(move |v| draft.revise(|old| *old = v)),
                            sapid::button()
                                .theme(content_theme.clone())
                                .label("Save profile")
                                .on_press(move || {
                                    saved.revise(|v| *v = draft.get());
                                    open.revise(|v| *v = false);
                                }),
                        ))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(saved.map(|v| format!("Current profile: {v}"))),
            ))
            .show_in(scope)
    })
}
