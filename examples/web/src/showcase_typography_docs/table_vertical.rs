use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::table("vertical-project")
        .theme(theme.clone())
        .caption("Project details")
        .with_row_borders(true)
        .body(
            sapid::table_body().rows(
                [
                    ("project", "Project", "Component library"),
                    ("owner", "Owner", "Jamie Chen"),
                    ("status", "Status", "In progress"),
                ]
                .into_iter()
                .map(|(key, label, value)| {
                    sapid::table_row(key).cells([
                        sapid::table_row_header("label").text(label),
                        sapid::table_cell("value").text(value),
                    ])
                }),
            ),
        )
}
