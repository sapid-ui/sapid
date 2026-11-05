use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage("Choose a workspace action.".to_owned());
        let icon = |kind: sapid::Icon| glory::ui::graphic().drawing(kind.drawing(theme.foreground))
            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.)).pointer_events(glory::style::PointerEvents::None));
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::dropdown_menu("workspace-menu").theme(theme.clone()).label("Toggle menu")
                .name("Workspace actions").width(240.)
                .item(sapid::menu_label("application", "Application"))
                .item(sapid::menu_item("calendar").label("Calendar")
                    .left_section(icon(sapid::Icon::Calendar))
                    .right_section(glory::ui::text().content("Ctrl+K"))
                    .on_activate(move || selected.revise(|old| *old = "Calendar selected".into())))
                .item(sapid::menu_item("about").label("About this workspace")
                    .left_section(icon(sapid::Icon::Info))
                    .on_activate(move || selected.revise(|old| *old = "Workspace information selected".into())))
                .separator("more-actions")
                .submenu(sapid::menu_submenu("export").label("Export notes").width(200.)
                    .item(sapid::menu_item("text").label("Plain text")
                        .on_activate(move || selected.revise(|old| *old = "Plain text export selected".into())))
                    .item(sapid::menu_item("markdown").label("Markdown")
                        .on_activate(move || selected.revise(|old| *old = "Markdown export selected".into())))),
            sapid::text().theme(theme.clone()).content(selected),
        )).show_in(scope)
    })
}
