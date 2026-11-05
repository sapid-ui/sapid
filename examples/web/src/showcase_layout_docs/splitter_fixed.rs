use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::flex()
        .theme(theme.clone())
        .gap(0.)
        .height(Length::Px(220.))
        .fill((
            sapid::box_()
                .theme(theme.clone())
                .style(Style {
                    shrink: Some(0.),
                    background: Some(theme.accent_soft),
                    ..Style::new()
                        .width(Length::Px(140.))
                        .max_width(Limit::Percent(35.))
                        .padding(Edges::all(10.))
                })
                .fill(
                    sapid::text()
                        .theme(theme.clone())
                        .content("Fixed navigation"),
                ),
            sapid::splitter("docs-fixed-splitter")
                .theme(theme.clone())
                .name("Resize flexible workspaces")
                .min(20.)
                .max(80.)
                .style(Style {
                    grow: Some(1.),
                    ..Style::new()
                        .min_width(Length::Px(0.))
                        .height(Length::Percent(100.))
                })
                .first({
                    let theme = theme.clone();
                    move || sapid::text().theme(theme.clone()).content("Editor")
                })
                .second(move || sapid::text().theme(theme.clone()).content("Preview")),
        ))
}
