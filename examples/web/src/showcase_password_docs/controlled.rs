use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(false);
        let accepted = scope.cage(true);
        let draft = scope.cage(String::new());
        let proposal = scope.cage("No visibility proposal yet".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept visibility proposals")
                    .checked(accepted)
                    .on_checked_change(move |v| accepted.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Reveal from an external control")
                    .checked(visible)
                    .on_checked_change(move |v| visible.revise(|old| *old = v)),
                sapid::field("controlled-password")
                    .theme(theme.clone())
                    .label("Controlled password")
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .value(draft)
                            .on_value_change(move |v| draft.revise(|old| *old = v))
                            .visible(visible)
                            .on_visibility_change(move |next| {
                                proposal.revise(|s| {
                                    *s = if next {
                                        "Requested reveal"
                                    } else {
                                        "Requested masking"
                                    }
                                    .into()
                                });
                                if accepted.get() {
                                    visible.revise(|old| *old = next);
                                }
                            })
                    }),
                sapid::text().theme(theme.clone()).content(proposal),
            ))
            .show_in(scope)
    })
}
