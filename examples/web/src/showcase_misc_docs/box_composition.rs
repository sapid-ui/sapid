use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let done = scope.cage(false);
        sapid::box_()
            .theme(theme.clone())
            .style(Style::new().padding(Edges::all(16.)))
            .fill(
                sapid::stack().theme(theme.clone()).gap(12.).fill((
                    sapid::text().theme(theme.clone()).content(done.map(|v| {
                        if *v {
                            "Your project brief is ready.".to_owned()
                        } else {
                            "Review the brief before sharing it with your team.".to_owned()
                        }
                    })),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Mark brief ready")
                        .on_press(move || done.revise(|v| *v = true)),
                )),
            )
            .show_in(scope)
    })
}
