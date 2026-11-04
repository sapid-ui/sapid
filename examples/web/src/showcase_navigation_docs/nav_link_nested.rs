use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(true);
        let children_theme = theme.clone();
        sapid::nav_link("account")
            .theme(theme.clone())
            .label("Account")
            .href("#account")
            .expanded(open)
            .on_expanded_change(move |next| open.revise(|old| *old = next))
            .content_policy(sapid::ContentPolicy::KeepMounted)
            .children(move || {
                sapid::stack()
                    .theme(children_theme.clone())
                    .gap(4.)
                    .style(Style::new().padding(Edges {
                        left: 20.,
                        right: 0.,
                        top: 4.,
                        bottom: 4.,
                    }))
                    .fill((
                        sapid::nav_link("profile")
                            .theme(children_theme.clone())
                            .label("Profile")
                            .href("#profile")
                            .current(true),
                        sapid::nav_link("security")
                            .theme(children_theme.clone())
                            .label("Security")
                            .href("#security"),
                        sapid::text_field()
                            .theme(children_theme.clone())
                            .name("Account note")
                            .placeholder("A note is retained when collapsed"),
                    ))
            })
            .show_in(scope)
    })
}
