use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(vec!["email".to_owned()]);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox_group("notification-channels")
                    .theme(theme.clone())
                    .label("Notification channels")
                    .select_all_label("All available channels")
                    .selected(selected)
                    .items([
                        sapid::checkbox_item("email", "Email"),
                        sapid::checkbox_item("sms", "SMS"),
                        sapid::checkbox_item("push", "Push notifications").enabled(false),
                    ])
                    .on_selected_change(move |v| selected.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|v| format!("Selected: {}", v.join(", ")))),
            ))
            .show_in(scope)
    })
}
