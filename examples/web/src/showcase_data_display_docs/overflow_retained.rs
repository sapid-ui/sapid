use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let collapsed = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Move note into More")
                    .checked(collapsed)
                    .on_checked_change(move |v| collapsed.revise(|s| *s = v)),
                sapid::overflow_list("retained-project-note")
                    .theme(theme.clone())
                    .visible_count(collapsed.map(|v| if *v { 1usize } else { 2usize }))
                    .overflow_label("More")
                    .item(sapid::overflow_item("summary").width(90.).content({
                        let t = theme.clone();
                        move || sapid::badge().theme(t.clone()).content("Project")
                    }))
                    .item(sapid::overflow_item("note").width(180.).content({
                        let t = theme.clone();
                        move || {
                            sapid::field("retained-project-note")
                                .theme(t.clone())
                                .label("Retained project note")
                                .control(|field| {
                                    sapid::text_field().field(field).placeholder("Write a note")
                                })
                        }
                    })),
            ))
            .show_in(scope)
    })
}
