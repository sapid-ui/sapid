use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(500_000usize);
        let inner = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::text().theme(theme.clone()).content("A million logical pages, with current ± 1 mounted. Nearby moves retain local drafts; jumping away retires their Owners."),
            sapid::button().theme(theme.clone()).label("Jump between start and middle").on_press(move||current.revise(|old|*old=if *old<100{500_000}else{0})),
            sapid::carousel().theme(theme.clone()).name("Windowed notes").count(1_000_000usize).current(current).overscan(1).with_indicators(true)
                .on_current_change(move|next|current.revise(|old|*old=next))
                .render_slide(move|index|sapid::field(format!("page-{index}")).theme(inner.clone()).label(format!("Draft for page {}",index+1))
                    .style(Style::new().min_height(Length::Px(150.)).padding(Edges{top:16.,bottom:24.,left:52.,right:52.}))
                    .control(|field|sapid::text_field().field(field).default_value("A local slide draft"))),
        )).show_in(scope)
    })
}
