use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(vec![false, false, false]);
        let notice = scope.cage("Choose projects for a bulk action.".to_owned());
        let items = ["Website", "Design system", "Mobile app"]
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                sapid::checkbox()
                    .theme(theme.clone())
                    .label(name)
                    .checked(selected.map(move |v| v[i]))
                    .on_checked_change(move |value| selected.revise(|v| v[i] = value))
            })
            .collect::<Vec<_>>();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::stack().theme(theme.clone()).gap(10.).fill(items),
                sapid::action_bar("project-selection")
                    .theme(theme.clone())
                    .name("Selected project actions")
                    .selected_count(selected.map(|v| v.iter().filter(|x| **x).count()))
                    .group(
                        sapid::toolbar_group("project-commands")
                            .button(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Archive selected")
                                    .on_press(move || {
                                        let count =
                                            selected.with(|v| v.iter().filter(|x| **x).count());
                                        notice.revise(|v| {
                                            *v = format!("Archived {count} selected projects.")
                                        });
                                        selected.revise(|v| v.fill(false));
                                    }),
                            )
                            .button(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Delete selected")
                                    .enabled(false),
                            ),
                    )
                    .on_clear_selection(move || selected.revise(|v| v.fill(false))),
                sapid::text().theme(theme.clone()).content(notice),
            ))
            .show_in(scope)
    })
}
