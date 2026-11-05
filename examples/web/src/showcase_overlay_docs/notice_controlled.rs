use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let allow = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Notice background draft")
                    .default_value("Continue writing"),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow notice closing")
                    .checked(allow)
                    .on_checked_change(move |v| allow.revise(|old| *old = v)),
                sapid::floating_dialog("notice-control")
                    .theme(theme.clone())
                    .label("Show controlled notice")
                    .title("Background save completed")
                    .close_label("Close controlled notice")
                    .open(open)
                    .on_open_change(move |v| {
                        if v || allow.get() {
                            open.revise(|old| *old = v)
                        }
                    })
                    .description("You may continue using the background editor."),
            ))
            .show_in(scope)
    })
}
