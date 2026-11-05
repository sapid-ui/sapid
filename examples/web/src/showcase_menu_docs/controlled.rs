use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let accept = scope.cage(true);
        let status = scope.cage("The application controls this menu.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept menu open and close requests")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Open controlled menu")
                        .on_press(move || open.revise(|old| *old = true)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Close controlled menu")
                        .on_press(move || open.revise(|old| *old = false)),
                )),
                sapid::dropdown_menu("controlled-actions")
                    .theme(theme.clone())
                    .label("Controlled actions")
                    .open(open)
                    .on_open_change(move |next| {
                        if accept.get() {
                            open.revise(|old| *old = next);
                        }
                        status.revise(|old| {
                            *old = if accept.get() {
                                "Menu request accepted"
                            } else {
                                "Menu request declined"
                            }
                            .into()
                        });
                    })
                    .item(
                        sapid::menu_item("save")
                            .label("Save and close")
                            .on_activate(move || {
                                status.revise(|old| *old = "Save selected".into())
                            }),
                    )
                    .item(
                        sapid::menu_item("preview")
                            .label("Preview without closing")
                            .close_on_select(false)
                            .on_activate(move || {
                                status.revise(|old| {
                                    *old = "Preview selected; the menu stays open".into()
                                })
                            }),
                    ),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
