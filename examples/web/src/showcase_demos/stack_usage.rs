use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(Vec::from(
        [
            "Create a project",
            "Invite your teammates",
            "Share your first milestone",
        ]
        .map(|label| {
            sapid::card()
                .theme(theme.clone())
                .fill(sapid::text().theme(theme.clone()).content(label))
        }),
    ))
}
