use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        glory::ui::view().fill((
            sapid::text()
                .theme(theme.clone())
                .inline(true)
                .content("Explicit: "),
            sapid::mark().theme(theme.clone()).content("review ready"),
        )),
        glory::ui::view().fill(
            sapid::highlight()
                .theme(theme.clone())
                .content("The review is ready for the next project.")
                .query("review"),
        ),
    ))
}
