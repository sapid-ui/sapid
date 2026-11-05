use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage("storm".to_owned());
        let empty = query.map(|q| matches(q) == 0);
        let empty_theme = theme.clone();
        let result_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Search projects")
                    .value(query)
                    .on_value_change(move |v| query.revise(|old| *old = v)),
                when(empty, move || {
                    sapid::empty_state()
                        .theme(empty_theme.clone())
                        .title("No matching projects")
                        .description("Try another search or clear the current query.")
                        .illustration(
                            glory::ui::graphic()
                                .drawing(sapid::Icon::Info.drawing(empty_theme.muted))
                                .style(Style::new().width(Length::Px(36.)).height(Length::Px(36.))),
                        )
                        .actions(
                            sapid::button()
                                .theme(empty_theme.clone())
                                .label("Reset project search")
                                .on_press(move || query.revise(String::clear)),
                        )
                }),
                when(query.map(|q| matches(q) > 0), move || {
                    sapid::text()
                        .theme(result_theme.clone())
                        .content(query.map(|q| format!("{} projects match", matches(q))))
                }),
            ))
            .show_in(scope)
    })
}
fn matches(query: &str) -> usize {
    let query = query.trim().to_lowercase();
    ["Design system", "Website", "Mobile app"]
        .into_iter()
        .filter(|name| name.to_lowercase().contains(&query))
        .count()
}
