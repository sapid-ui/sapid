use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(false);
        let archived = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Select project report")
                    .checked(selected)
                    .on_checked_change(move |v| selected.revise(|old| *old = v)),
                sapid::action_bar("report-selection")
                    .theme(theme.clone())
                    .selected_count(selected.map(|v| usize::from(*v)))
                    .group(
                        sapid::toolbar_group("report-actions")
                            .button(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Archive report")
                                    .on_press(move || {
                                        archived.revise(|v| *v = true);
                                        selected.revise(|v| *v = false);
                                    }),
                            )
                            .button(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Restricted export")
                                    .enabled(false),
                            ),
                    )
                    .on_clear_selection(move || selected.revise(|v| *v = false)),
                sapid::text()
                    .theme(theme.clone())
                    .content(archived.map(|v| {
                        if *v {
                            "Report archived.".to_owned()
                        } else {
                            "The report is ready for selection.".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}
