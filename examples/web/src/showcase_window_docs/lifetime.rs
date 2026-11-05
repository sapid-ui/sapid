use glory::prelude::*;
use std::boxed::Box;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(None::<&'static str>);
        let mut examples: Vec<Box<dyn Widget>> = Vec::new();
        for (key, title, label, policy) in [
            (
                "retained-notes",
                "Retained notes",
                "Open retained notes",
                sapid::ContentPolicy::KeepMounted,
            ),
            (
                "fresh-notes",
                "Fresh notes",
                "Open fresh notes",
                sapid::ContentPolicy::Unmount,
            ),
        ] {
            examples.push(Box::new(
                sapid::button()
                    .theme(theme.clone())
                    .label(label)
                    .on_press(move || active.revise(|old| *old = Some(key))),
            ));
            let inner = theme.clone();
            examples.push(Box::new(
                sapid::floating_window(key)
                    .theme(theme.clone())
                    .title(title)
                    .open(active.map(move |value| *value == Some(key)))
                    .on_open_change(move |next| {
                        if !next && active.get() == Some(key) {
                            active.revise(|old| *old = None);
                        }
                    })
                    .default_bounds(sapid::WindowBounds {
                        x: 32.,
                        y: 96.,
                        width: 300.,
                        height: 240.,
                    })
                    .content_policy(policy)
                    .content(move || {
                        sapid::field(key)
                            .theme(inner.clone())
                            .label(title)
                            .control(|field| {
                                sapid::textarea()
                                    .field(field)
                                    .default_value("Edit this note, close and reopen.")
                            })
                    }),
            ));
        }
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill(examples)
            .show_in(scope)
    })
}
