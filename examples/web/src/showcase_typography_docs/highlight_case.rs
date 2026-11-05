use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        glory::ui::view().fill(
            sapid::highlight()
                .theme(theme.clone())
                .content("Design · design · DESIGN")
                .query("design"),
        ),
        glory::ui::view().fill(
            sapid::highlight()
                .theme(theme.clone())
                .content("团队设计指南 · café notes · 设计评审")
                .query("设计"),
        ),
    ))
}
