use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let created = scope.cage(false);
        let empty_theme = theme.clone();
        let ready_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                when(created.map(|v| !*v), move || {
                    sapid::empty_state()
                        .theme(empty_theme.clone())
                        .size(sapid::Size::Large)
                        .illustration(
                            glory::ui::graphic()
                                .drawing(sapid::Icon::Info.drawing(empty_theme.muted))
                                .style(Style::new().width(Length::Px(44.)).height(Length::Px(44.))),
                        )
                        .title("No projects yet")
                        .description(
                            "Create your first project to keep your team's work in one place.",
                        )
                        .actions((
                            sapid::button()
                                .theme(empty_theme.clone())
                                .label("Create a project")
                                .on_press(move || created.revise(|v| *v = true)),
                            sapid::button()
                                .theme(empty_theme.clone())
                                .variant(sapid::ButtonVariant::Outline)
                                .label("Use a sample project")
                                .on_press(move || created.revise(|v| *v = true)),
                        ))
                }),
                when(created, move || {
                    sapid::text()
                        .theme(ready_theme.clone())
                        .content("Your first project is ready.")
                }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Reset project example")
                    .enabled(created)
                    .on_press(move || created.revise(|v| *v = false)),
            ))
            .show_in(scope)
    })
}
