use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(1_u64);
        sapid::pagination("unknown-result-set")
            .theme(theme.clone())
            .page(page)
            .total(None)
            .has_next(scope.bond(move || page.get() < 4))
            .on_page_change(move |next| page.revise(|old| *old = next))
            .show_in(scope)
    })
}
