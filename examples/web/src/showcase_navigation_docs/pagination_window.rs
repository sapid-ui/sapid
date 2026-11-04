use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(10_u64);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        format!(
                            "Results {}–{} of 1000",
                            (page.get() - 1) * 10 + 1,
                            page.get() * 10
                        )
                    })),
                sapid::pagination("large-result-set")
                    .theme(theme.clone())
                    .page(page)
                    .total(Some(100))
                    .siblings(1)
                    .boundaries(1)
                    .on_page_change(move |next| page.revise(|old| *old = next)),
            ))
            .show_in(scope)
    })
}
