use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("Ada".to_owned());
        let accept = scope.cage(false);
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::field("text-controlled")
                    .theme(theme.clone())
                    .label("Controlled name")
                    .readonly(readonly)
                    .enabled(disabled.map(|v| !*v))
                    .control(move |field| {
                        sapid::text_field()
                            .field(field)
                            .value(value)
                            .on_value_change(move |next| {
                                if accept.get() {
                                    value.revise(|v| *v = next)
                                }
                            })
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept text changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Make text read-only")
                    .checked(readonly)
                    .on_checked_change(move |v| readonly.revise(|old| *old = v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Disable text editor")
                    .checked(disabled)
                    .on_checked_change(move |v| disabled.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Set name from application")
                    .on_press(move || value.revise(|v| *v = "Grace".into())),
            ))
            .show_in(scope)
    })
}
