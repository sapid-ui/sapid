use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroller("vertical-notes")
        .axis(Orientation::Vertical)
        .area(|area| {
            area.theme(theme.clone())
                .name("Project notes")
                .style(Style::new().height(Length::Px(180.)))
        })
        .content(move |_| {
            sapid::stack().theme(theme.clone()).gap(12.).fill(
                (1..=10)
                    .map(|index| {
                        sapid::paper()
                            .theme(theme.clone())
                            .style(Style::new().padding(Edges::all(16.)))
                            .fill(text().content(format!(
                                "Project note {index}: review the next milestone."
                            )))
                    })
                    .collect::<Vec<_>>(),
            )
        })
}
