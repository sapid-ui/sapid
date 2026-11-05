use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(sapid::CopyState::Idle);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::copy_button()
                    .theme(theme.clone())
                    .source("https://example.com/invite/team")
                    .label("Copy invite link")
                    .copied_label("Link copied")
                    .failed_label("Could not copy link")
                    .state_ref(state),
                sapid::text()
                    .theme(theme.clone())
                    .content(state.map(|v| format!("Clipboard result: {v:?}"))),
            ))
            .show_in(scope)
    })
}
