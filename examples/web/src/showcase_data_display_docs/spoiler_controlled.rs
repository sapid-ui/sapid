use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let allow = scope.cage(false);
        let expanded = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow guide expansion")
                    .checked(allow)
                    .on_checked_change(move |v| allow.revise(|s| *s = v)),
                sapid::spoiler("controlled-guide")
                    .theme(theme.clone())
                    .max_height(64.)
                    .labels("Read more about the route", "Collapse route details")
                    .content(TEXT_1)
                    .expanded(expanded)
                    .on_expanded_change(move |v| {
                        if allow.get() {
                            expanded.revise(|s| *s = v)
                        }
                    }),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "The trail begins beside the lake, where a short boardwalk leads into the old village. Follow the stone steps to a quiet lookout above the rooftops.\n\nIn the morning the mountains reflect in the water and small boats cross toward the opposite shore. Leave time for the shaded woodland path and a stop at the village bakery.\n\nThe final stretch climbs gently through the pines before returning to the lakeside. Comfortable shoes, a refillable water bottle and a light jacket are useful for the changing mountain weather.";
