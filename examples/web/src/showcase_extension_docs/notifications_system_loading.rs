use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let busy = scope.cage(true);
        let visible = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Uploading attachment")
                    .checked(busy)
                    .on_checked_change(move |v| busy.revise(|o| *o = v)),
                sapid::notification("attachment-upload")
                    .theme(theme.clone())
                    .title("Attachment")
                    .content(scope.bond(move || {
                        if busy.get() {
                            "Uploading attachment…".to_string()
                        } else {
                            "Attachment uploaded".into()
                        }
                    }))
                    .loading(busy)
                    .visible(visible)
                    .closable(true)
                    .close_label("Dismiss upload result")
                    .on_close(move || visible.revise(|o| *o = false)),
            ))
            .show_in(scope)
    })
}
