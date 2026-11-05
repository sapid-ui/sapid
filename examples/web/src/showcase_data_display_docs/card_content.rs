use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let expanded = scope.cage(false);
        sapid::card()
            .theme(theme.clone())
            .title("A project title with enough detail to wrap on a narrow screen")
            .fill(
                sapid::text()
                    .theme(theme.clone())
                    .content(expanded.map(|v| {
                        if *v {
                            TEXT_1.to_owned()
                        } else {
                            "A shared component library for the whole team.".to_owned()
                        }
                    })),
            )
            .footer(
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle project details")
                    .on_press(move || expanded.revise(|v| *v = !*v)),
            )
            .show_in(scope)
    })
}

const TEXT_1: &str = "The design system brings together accessible components, shared tokens, complete public examples and clear platform support boundaries. The team can review each change before the next release.";
