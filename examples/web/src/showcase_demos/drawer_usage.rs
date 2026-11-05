use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body_theme = theme.clone();
    sapid::drawer("workspace-drawer")
        .theme(theme)
        .label("Open workspace drawer")
        .title("Your workspace")
        .description("Manage the workspace without leaving the current page.")
        .close_label("Close workspace drawer")
        .side(sapid::DrawerSide::Right)
        .style(Style::new().width(Length::Px(340.)))
        .content_policy(sapid::ContentPolicy::KeepMounted)
        .content(move || {
            widget({
                let theme = body_theme.clone();
                move |scope| {
                    let note = scope.cage("Team launch".to_owned());
                    let notifications = scope.cage(true);
                    sapid::stack()
                        .theme(theme.clone())
                        .gap(16.)
                        .fill((
                            sapid::text_field()
                                .theme(theme.clone())
                                .name("Workspace project note")
                                .value(note)
                                .on_value_change(move |v| note.revise(|old| *old = v)),
                            sapid::checkbox()
                                .theme(theme.clone())
                                .label("Workspace updates")
                                .checked(notifications)
                                .on_checked_change(move |v| notifications.revise(|old| *old = v)),
                            sapid::text().theme(theme.clone()).content(
                                "Your draft is retained when you close and reopen this drawer.",
                            ),
                        ))
                        .show_in(scope)
                }
            })
        })
}
