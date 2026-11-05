use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let draft_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        sapid::color_input("showcase-color-invalid-draft")
            .theme(draft_theme.clone())
            .value(value)
            .fix_on_blur(false)
            .on_value_change(move |next| value.revise(|old| *old = next))
            .show_in(scope)
    })
}
