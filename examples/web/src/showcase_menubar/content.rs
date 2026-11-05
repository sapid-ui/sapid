use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let status = scope.cage("Choose an application action".to_owned());
        let calendar = glory::ui::graphic()
            .drawing(sapid::Icon::Calendar.drawing(theme.foreground))
            .style(
                Style::new()
                    .width(Length::Px(16.))
                    .height(Length::Px(16.))
                    .pointer_events(glory::style::PointerEvents::None),
            );
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::menubar("content-bar")
                    .theme(theme.clone())
                    .name("Application actions")
                    .menu(
                        sapid::menubar_menu("workspace")
                            .label("Workspace")
                            .width(260.)
                            .item(sapid::menu_label("navigate", "Navigate"))
                            .item(
                                sapid::menu_item("calendar")
                                    .label("Open calendar")
                                    .left_section(calendar)
                                    .right_section(glory::ui::text().content("Ctrl+K"))
                                    .on_activate(move || {
                                        status.revise(|old| *old = "Calendar opened".into())
                                    }),
                            )
                            .separator("exports")
                            .item(sapid::menu_label("export", "Export document"))
                            .submenu(
                                sapid::menu_submenu("format")
                                    .label("Choose a format")
                                    .width(180.)
                                    .right_section(glory::ui::text().content("›"))
                                    .item(sapid::menu_item("text").label("Plain text").on_activate(
                                        move || {
                                            status.revise(|old| *old = "Plain text selected".into())
                                        },
                                    ))
                                    .item(
                                        sapid::menu_item("markdown").label("Markdown").on_activate(
                                            move || {
                                                status
                                                    .revise(|old| *old = "Markdown selected".into())
                                            },
                                        ),
                                    ),
                            ),
                    ),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
