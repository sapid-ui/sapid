use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let concise = scope.cage(false);
        let result = scope.cage("Choose an action".to_owned());
        let icon = glory::ui::graphic()
            .drawing(sapid::Icon::Check.drawing(theme.foreground))
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
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use concise labels")
                    .checked(concise)
                    .on_checked_change(move |next| concise.revise(|old| *old = next)),
                sapid::dropdown_menu("content-actions")
                    .theme(theme.clone())
                    .label("Document actions")
                    .width(260.)
                    .item(sapid::menu_label(
                        "document",
                        concise.map(|concise| {
                            if *concise {
                                "Draft"
                            } else {
                                "Current document"
                            }
                            .to_owned()
                        }),
                    ))
                    .item(
                        sapid::menu_item("save")
                            .label(concise.map(|concise| {
                                if *concise {
                                    "Save"
                                } else {
                                    "Save current draft"
                                }
                                .to_owned()
                            }))
                            .left_section(icon)
                            .right_section(glory::ui::text().content("Ctrl+S"))
                            .on_activate(move || result.revise(|old| *old = "Draft saved".into())),
                    )
                    .item(
                        sapid::menu_item("share")
                            .label("Share with your team")
                            .right_section(glory::ui::text().content("Ctrl+Shift+S"))
                            .on_activate(move || {
                                result.revise(|old| *old = "Sharing selected".into())
                            }),
                    )
                    .separator("export-actions")
                    .submenu(
                        sapid::menu_submenu("export").label("Export").item(
                            sapid::menu_item("pdf")
                                .label("PDF document")
                                .on_activate(move || {
                                    result.revise(|old| *old = "PDF export selected".into())
                                }),
                        ),
                    ),
                sapid::text().theme(theme.clone()).content(result),
            ))
            .show_in(scope)
    })
}
