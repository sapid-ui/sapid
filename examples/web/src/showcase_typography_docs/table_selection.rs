use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::table("selected-project")
                    .theme(theme.clone())
                    .caption("Project review")
                    .head(sapid::table_head().row(sapid::table_row("head").cells([
                        sapid::table_header_cell("select").text("Review"),
                        sapid::table_header_cell("project").text("Project"),
                    ])))
                    .body(sapid::table_body().row(sapid::table_row("website").cells([
                        sapid::table_cell("select").content({
                            let theme = theme.clone();
                            move || {
                                sapid::checkbox()
                                    .theme(theme.clone())
                                    .label("Select website project")
                                    .checked(selected)
                                    .on_checked_change(move |v| selected.revise(|s| *s = v))
                            }
                        }),
                        sapid::table_row_header("project").text("Website redesign"),
                    ]))),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|s| {
                        if *s {
                            "1 project selected for review".to_owned()
                        } else {
                            "No projects selected".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}
