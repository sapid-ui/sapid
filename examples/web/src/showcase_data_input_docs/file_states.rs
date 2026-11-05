use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let disabled = scope.cage(false);
        sapid::stack().theme(theme.clone()).gap(16.).fill((
            sapid::checkbox().theme(theme.clone()).label("Disable attachment picker").checked(disabled)
                .on_checked_change(move|v|disabled.revise(|old|*old=v)),
            sapid::file_input().theme(theme.clone()).name("Optional attachment").label("Choose an optional attachment")
                .enabled(disabled.map(|v|!*v)).clearable(true).show_attachments(false),
            sapid::text().theme(theme.clone()).content("Cancelling a new chooser preserves the previous accepted selection. A rejected selection reports its reason. Clearing releases the current file references."),
        )).show_in(scope)
    })
}
