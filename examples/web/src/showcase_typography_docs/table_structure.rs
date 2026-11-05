use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::scroll_area("structure-table-scroll")
        .axes(glory::render::ScrollAxes::Both)
        .theme(theme.clone())
        .name("Project table structure")
        .style(
            Style::new()
                .height(Length::Px(310.))
                .max_width(glory::style::Limit::Percent(100.)),
        )
        .content(move |_| {
            projects(&theme, "structure-projects")
                .caption_side(sapid::TableCaptionSide::Bottom)
                .style(Style::new().min_width(Length::Px(560.)))
                .foot(
                    sapid::table_foot().row(
                        sapid::table_row("summary").cells([
                            sapid::table_row_header("label").text("Total projects"),
                            sapid::table_cell("value")
                                .column_span(3)
                                .text("4 active studio projects"),
                        ]),
                    ),
                )
        })
}

fn projects(theme: &sapid::Theme, key: &str) -> sapid::Table {
    sapid::table(key)
        .theme(theme.clone())
        .caption("Studio projects")
        .head(sapid::table_head().row(sapid::table_row("head").cells([
            sapid::table_header_cell("project").text("Project"),
            sapid::table_header_cell("owner").text("Owner"),
            sapid::table_header_cell("status").text("Status"),
            sapid::table_header_cell("due").text("Due date"),
        ])))
        .body(
            sapid::table_body().rows(
                [
                    (
                        "website",
                        "Website redesign",
                        "Ava Patel",
                        "In review",
                        "Nov 18",
                    ),
                    (
                        "library",
                        "Component library",
                        "Jamie Chen",
                        "In progress",
                        "Nov 24",
                    ),
                    (
                        "research",
                        "Research notes",
                        "Sam Rivera",
                        "Ready",
                        "Nov 28",
                    ),
                    (
                        "mobile",
                        "Mobile prototype",
                        "Robin Park",
                        "Planning",
                        "Dec 02",
                    ),
                ]
                .into_iter()
                .map(|(key, project, owner, status, due)| {
                    sapid::table_row(key).cells([
                        sapid::table_row_header("project").text(project),
                        sapid::table_cell("owner").text(owner),
                        sapid::table_cell("status").text(status),
                        sapid::table_cell("due").text(due),
                    ])
                }),
            ),
        )
}
