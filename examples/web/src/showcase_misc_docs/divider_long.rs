use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let long = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::separator()
                    .theme(theme.clone())
                    .label(long.map(|v| {
                        if *v {
                            TEXT_1.to_owned()
                        } else {
                            "Other options".to_owned()
                        }
                    }))
                    .variant(sapid::SeparatorVariant::Dashed),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use a longer separator label")
                    .checked(long)
                    .on_checked_change(move |v| long.revise(|s| *s = v)),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "More ways to continue with the account you use for your shared workspace";
