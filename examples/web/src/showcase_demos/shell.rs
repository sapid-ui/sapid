use glory::prelude::*;
use glory::render::Current;
use glory::style::{Edges, Length, Style};

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage("Overview".to_owned());
        let nav_theme = theme.clone();
        let header_theme = theme.clone();
        let main_theme = theme.clone();
        let footer_theme = theme.clone();
        sapid::app_shell("showcase-workspace")
            .theme(theme.clone())
            .class("site-shell-example")
            .breakpoint(560.)
            .style(Style::new().min_height(Length::Px(420.)))
            .header(move |_| {
                sapid::group().theme(header_theme.clone()).fill((
                    sapid::text()
                        .theme(header_theme.clone())
                        .content("Acme workspace")
                        .style(Style {
                            font_weight: Some(650.),
                            ..Style::new()
                        }),
                    sapid::badge()
                        .theme(header_theme.clone())
                        .content("Personal"),
                ))
            })
            .sidebar(
                sapid::sidebar("showcase-workspace-nav")
                    .name("Workspace navigation")
                    .width(200.)
                    .content(move |context| {
                        let collapsed = context.sidebar.collapsed();
                        let nav_theme = nav_theme.clone();
                        widget(move |scope| {
                            let mut links = Vec::new();
                            for (label, short) in [
                                ("Overview", "O"),
                                ("Projects", "P"),
                                ("Team", "T"),
                                ("Settings", "S"),
                            ] {
                                let current = selected
                                    .map(move |value| (value == label).then_some(Current::Page));
                                let title = scope.bond(move || {
                                    if collapsed.get() { short } else { label }.to_owned()
                                });
                                links.push(
                                    sapid::anchor()
                                        .theme(nav_theme.clone())
                                        .label(title)
                                        .href(format!("#{label}"))
                                        .current(current)
                                        .class("site-shell-nav-link")
                                        .on_navigate(move |_| {
                                            selected.revise(|value| *value = label.to_owned())
                                        }),
                                );
                            }
                            sapid::stack()
                                .theme(nav_theme.clone())
                                .gap(4.)
                                .style(Style::new().padding(Edges::all(12.)))
                                .fill(links)
                                .show_in(scope)
                        })
                    }),
            )
            .main(move |_| {
                let heading = selected;
                sapid::stack().theme(main_theme.clone()).gap(16.).fill((
                    sapid::title(3).theme(main_theme.clone()).content(heading),
                    sapid::text()
                        .theme(main_theme.clone())
                        .content("A place for your team's projects, people and ideas."),
                    sapid::card().theme(main_theme.clone()).fill(
                        sapid::stack().theme(main_theme.clone()).gap(12.).fill((
                            sapid::text()
                                .theme(main_theme.clone())
                                .content("Welcome to your workspace"),
                            sapid::text_field()
                                .theme(main_theme.clone())
                                .name("Workspace note")
                                .placeholder("Write a quick note…"),
                        )),
                    ),
                ))
            })
            .footer(move |_| {
                sapid::text()
                    .theme(footer_theme.clone())
                    .content("Acme workspace · All changes saved")
                    .style(Style {
                        font_size: Some(12.),
                        ..Style::new()
                    })
            })
            .show_in(scope)
    })
}
