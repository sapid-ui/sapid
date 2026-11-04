use glory::prelude::*;
use glory::render::{FileConfig, Orientation};
use glory::style::{FlexWrap, ObjectFit};
use glory::ui::text;
use sapid::prelude::*;
use sapid::{
    Icon, ThemeIconVariant, autocomplete_option, background_image, button, file_button, file_input,
    group, image, indicator, progress, ring_progress, select_group, select_option,
    semi_circle_progress, theme_icon,
};
use std::{boxed::Box, cell::RefCell, rc::Rc, time::Duration};

fn tile(theme: &Theme, label: &str) -> sapid::Box {
    sapid::box_()
        .theme(theme.clone())
        .style(Style {
            background: Some(theme.accent_soft),
            color: Some(theme.accent_text),
            ..Style::new().padding(Edges::all(16.0)).radius(theme.radius)
        })
        .fill(text().content(label.to_owned()))
}

fn media(name: &str) -> String {
    format!(
        "{}/{}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets"),
        name
    )
}

fn date(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}

fn datetime(value: &str) -> LocalDateTime {
    parse_local_date_time(value).unwrap()
}

fn schedule_data(window: ScheduleWindow) -> ScheduleData {
    ScheduleData::new(
        ScheduleZone::Civil,
        window,
        [
            ScheduleEvent::new(
                ScheduleKey::new("planning").unwrap(),
                "Planning",
                ScheduleInterval::new(datetime("2024-02-29T09:00"), datetime("2024-02-29T10:00"))
                    .unwrap(),
            )
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("standup").unwrap(),
                "Daily standup",
                ScheduleInterval::new(datetime("2024-02-29T09:30"), datetime("2024-02-29T10:00"))
                    .unwrap(),
            )
            .unwrap()
            .recurring(
                ScheduleRecurrence::new(RecurrenceFrequency::Daily, 1, RecurrenceEnd::Never)
                    .unwrap(),
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn resource_data(window: ScheduleWindow) -> ResourceScheduleData {
    let key = |value: &str| ScheduleKey::new(value).unwrap();
    let event = |id: &str, title: &str, resource: &str| {
        ScheduleEvent::new(
            key(id),
            title,
            ScheduleInterval::new(datetime("2024-02-29T09:00"), datetime("2024-02-29T10:00"))
                .unwrap(),
        )
        .unwrap()
        .resource(key(resource))
    };
    ResourceScheduleData::new(
        ScheduleData::new(
            ScheduleZone::Civil,
            window,
            [
                event("planning", "Planning", "room-a"),
                event("review", "Review", "room-b"),
            ],
        )
        .unwrap(),
        ScheduleResources::new([
            ScheduleResource::new(key("room-a"), "Room A").unwrap(),
            ScheduleResource::new(key("room-b"), "Room B").unwrap(),
        ])
        .unwrap(),
        ScheduleResourceWindow::new(0, 2).unwrap(),
    )
    .unwrap()
}

fn toolbar_group(theme: &Theme, key: &str) -> ToolbarGroup {
    sapid::toolbar_group(key).name("Formatting").buttons([
        sapid::button().theme(theme.clone()).label("Bold"),
        sapid::button().theme(theme.clone()).label("Italic"),
    ])
}

fn progress_example(name: &str, theme: Theme) -> Box<dyn Widget> {
    let name = name.to_owned();
    Box::new(widget(move |scope| {
        let value = scope.cage(64.0);
        let value_text = value.map(|value| format!("{value:.0}%"));
        let preview: Box<dyn Widget> = match name.as_str() {
            "Progress" => Box::new(
                progress(value)
                    .theme(theme.clone())
                    .name("File upload")
                    .value_text(value_text)
                    .size(240.0)
                    .thickness(10.0),
            ),
            "RingProgress" => Box::new(
                ring_progress(value)
                    .theme(theme.clone())
                    .name("Circular upload")
                    .value_text(value_text)
                    .size(120.0)
                    .thickness(10.0),
            ),
            _ => Box::new(
                semi_circle_progress(value)
                    .theme(theme.clone())
                    .name("Semicircular upload")
                    .value_text(value_text)
                    .size(160.0)
                    .thickness(10.0),
            ),
        };
        sapid::stack()
            .theme(theme.clone())
            .gap(16.0)
            .fill((
                preview,
                sapid::slider("progress-value")
                    .theme(theme.clone())
                    .name("Progress value")
                    .min(0.0)
                    .max(100.0)
                    .step(1.0)
                    .value(value)
                    .on_value_change(move |next| value.revise(|current| *current = next)),
            ))
            .show_in(scope)
    }))
}

const RUST_EXAMPLE: &str = "pub fn greet(name: &str) -> String {\n    // Unicode stays plain code: 😀 <script>\n    format!(\"Hello, {name}!\")\n}\n";
const JSON_EXAMPLE: &str =
    "{\n  \"name\": \"Sapid\",\n  \"ready\": true,\n  \"values\": [1, 2, 3]\n}\n";

fn code_highlight_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let source = scope.cage(RUST_EXAMPLE.to_owned());
        let language = scope.cage(CodeLanguage::Rust);
        let expanded = scope.cage(false);
        let clipboard = scope.cage(CopyState::Idle);
        let visible_source = scope.bond(move || {
            let source = source.get();
            if expanded.get() {
                source
            } else {
                source.lines().take(3).collect::<Vec<_>>().join("\n")
            }
        });
        let label = expanded.map(|expanded| {
            if *expanded {
                "Collapse code".to_owned()
            } else {
                "Expand code".to_owned()
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(12.0)
            .fill((
                sapid::code_highlight()
                    .theme(theme.clone())
                    .class("site-code-highlight-demo")
                    .name("Rust and JSON code")
                    .source(visible_source)
                    .language(language)
                    .line_numbers(true)
                    .highlighted_lines([2])
                    .copy_button(
                        sapid::copy_button()
                            .theme(theme.clone())
                            .label("Copy code")
                            .copied_label("Copied")
                            .state_ref(clipboard),
                    ),
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Rust example")
                        .on_press(move || {
                            source.revise(|value| *value = RUST_EXAMPLE.into());
                            language.revise(|value| *value = CodeLanguage::Rust);
                            expanded.revise(|value| *value = false);
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .label("JSON example")
                        .on_press(move || {
                            source.revise(|value| *value = JSON_EXAMPLE.into());
                            language.revise(|value| *value = CodeLanguage::Json);
                            expanded.revise(|value| *value = true);
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .label(label)
                        .on_press(move || expanded.revise(|value| *value = !*value)),
                )),
                sapid::text()
                    .theme(theme.clone())
                    .variant(TextVariant::Small)
                    .content(clipboard.map(|state| format!("Clipboard status: {state:?}"))),
            ))
            .show_in(scope)
    })
}

const ELEMENTS: &[(&str, &str, &str, &str)] = &[
    ("6", "Carbon", "C", "12.011"),
    ("7", "Nitrogen", "N", "14.007"),
    ("39", "Yttrium", "Y", "88.906"),
    ("56", "Barium", "Ba", "137.335"),
    ("58", "Cerium", "Ce", "140.12"),
];

fn element_table(theme: &Theme, key: &str) -> Table {
    table(key)
        .theme(theme.clone())
        .caption("Some elements from the periodic table")
        .head(table_head().row(table_row("head").cells([
            table_header_cell("position").text("Element position"),
            table_header_cell("name").text("Element name"),
            table_header_cell("symbol").text("Symbol"),
            table_header_cell("mass").text("Atomic mass"),
        ])))
        .body(
            table_body().rows(ELEMENTS.iter().map(|(position, name, symbol, mass)| {
                table_row(*name).cells([
                    table_cell("position").text(*position),
                    table_row_header("name").text(*name),
                    table_cell("symbol").text(*symbol),
                    table_cell("mass").text(*mass),
                ])
            })),
        )
}

fn table_demo(theme: Theme) -> impl Widget {
    let caption = element_table(&theme, "elements-caption")
        .caption_side(TableCaptionSide::Bottom)
        .foot(table_foot().row(table_row("foot").cells([
            table_row_header("position").text("Element position"),
            table_row_header("name").text("Element name"),
            table_row_header("symbol").text("Symbol"),
            table_row_header("mass").text("Atomic mass"),
        ])));
    let styled = element_table(&theme, "elements-styled")
        .striped(true)
        .highlight_on_hover(true)
        .with_table_border(true)
        .with_column_borders(true)
        .with_row_borders(true);
    let spaced = element_table(&theme, "elements-spaced")
        .horizontal_spacing(24.0)
        .vertical_spacing(3.0);
    let vertical = table("vertical-details")
        .theme(theme.clone())
        .caption("Vertical variant")
        .with_row_borders(true)
        .body(
            table_body()
                .row(table_row("name").cells([
                    table_row_header("label").text("Epic name"),
                    table_cell("value").text("7.x migration"),
                ]))
                .row(table_row("status").cells([
                    table_row_header("label").text("Status"),
                    table_cell("value").text("Open"),
                ]))
                .row(table_row("issues").cells([
                    table_row_header("label").text("Total issues"),
                    table_cell("value").text("135"),
                ]))
                .row(table_row("points").cells([
                    table_row_header("label").text("Total story points"),
                    table_cell("value").text("874"),
                ])),
        );
    let scroll_theme = theme.clone();
    let sticky = scroll_area("elements-scroll")
        .theme(theme.clone())
        .name("Scrollable periodic table")
        .style(
            Style::new()
                .height(Length::Px(240.0))
                .max_width(Limit::Percent(100.0)),
        )
        .content(move |_| {
            element_table(&scroll_theme, "elements-sticky")
                .class("site-sticky-table")
                .sticky_header(true)
                .sticky_header_offset(Length::Px(0.0))
        });
    sapid::stack().theme(theme.clone()).gap(24.0).fill((
        sapid::title(3)
            .theme(theme.clone())
            .content("Caption, tfoot and sticky header"),
        caption,
        sapid::title(3)
            .theme(theme.clone())
            .content("Scroll container"),
        sticky,
        sapid::title(3).theme(theme.clone()).content("Spacing"),
        spaced,
        sapid::title(3)
            .theme(theme.clone())
            .content("Striped, hover and borders"),
        styled,
        sapid::title(3)
            .theme(theme.clone())
            .content("Vertical variant"),
        vertical,
        widget(move |scope| {
            let selected = scope.cage(Vec::<String>::new());
            let rows = ELEMENTS
                .iter()
                .map(|(position, name, symbol, mass)| {
                    let key = (*name).to_owned();
                    let checked = selected.map({
                        let key = key.clone();
                        move |values| values.contains(&key)
                    });
                    table_row(*name).cells([
                        table_cell("select").content({
                            let key = key.clone();
                            let theme = theme.clone();
                            move || {
                                sapid::checkbox()
                                    .theme(theme.clone())
                                    .label(format!("Select {name}"))
                                    .checked(checked)
                                    .on_checked_change({
                                        let key = key.clone();
                                        move |next| {
                                            selected.revise(|values| {
                                                if next && !values.contains(&key) {
                                                    values.push(key.clone());
                                                } else if !next {
                                                    values.retain(|value| value != &key);
                                                }
                                            })
                                        }
                                    })
                            }
                        }),
                        table_cell("position").text(*position),
                        table_row_header("name").text(*name),
                        table_cell("symbol").text(*symbol),
                        table_cell("mass").text(*mass),
                    ])
                })
                .collect::<Vec<_>>();
            sapid::stack()
                .theme(theme.clone())
                .gap(12.0)
                .fill((
                    sapid::title(3)
                        .theme(theme.clone())
                        .content("Row selection"),
                    table("element-selection")
                        .theme(theme.clone())
                        .caption("Select elements to include in the report")
                        .highlight_on_hover(true)
                        .with_row_borders(true)
                        .head(table_head().row(table_row("head").cells([
                            table_header_cell("select").text("Select"),
                            table_header_cell("position").text("Position"),
                            table_header_cell("name").text("Name"),
                            table_header_cell("symbol").text("Symbol"),
                            table_header_cell("mass").text("Atomic mass"),
                        ])))
                        .body(table_body().rows(rows)),
                    sapid::text()
                        .theme(theme.clone())
                        .content(selected.map(|values| {
                            if values.is_empty() {
                                "No elements selected".to_owned()
                            } else {
                                format!("Selected: {}", values.join(", "))
                            }
                        })),
                ))
                .show_in(scope)
        }),
    ))
}

fn stepper_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1usize);
        let selectable = scope.cage(true);
        let skip = scope.cage(false);
        let back_enabled = active.map(|active| *active > 0);
        let next_enabled = active.map(|active| *active < 3);
        let content_theme = theme.clone();
        let account = step("account").label("Account").content({
            let theme = content_theme.clone();
            move || {
                sapid::text()
                    .theme(theme.clone())
                    .content("Step 1 content: Create an account")
            }
        });
        let details = step("details")
            .label("Details")
            .description("Verify your email")
            .optional(true)
            .content({
                let theme = content_theme.clone();
                move || {
                    sapid::text()
                        .theme(theme.clone())
                        .content("Step 2 content: Verify email")
                }
            });
        let review = step("review").label("Review").content({
            let theme = content_theme.clone();
            move || {
                sapid::text()
                    .theme(theme.clone())
                    .content("Step 3 content: Confirm your details")
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(24.0)
            .fill((
                stepper("showcase-workflow")
                    .theme(theme.clone())
                    .class("site-stepper-demo")
                    .active(active)
                    .selectable(selectable)
                    .allow_skip(skip)
                    .on_step_change(move |next| active.revise(|value| *value = next))
                    .steps([account, details, review])
                    .completed({
                        let theme = content_theme.clone();
                        move || {
                            sapid::text()
                                .theme(theme.clone())
                                .content("Workflow completed")
                        }
                    }),
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Back")
                        .enabled(back_enabled)
                        .on_press(move || active.revise(|value| *value = value.saturating_sub(1))),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Next")
                        .enabled(next_enabled)
                        .on_press(move || active.revise(|value| *value = (*value + 1).min(3))),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Allow step selection")
                        .checked(selectable)
                        .on_checked_change(move |next| selectable.revise(|value| *value = next)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Allow skipping steps")
                        .checked(skip)
                        .on_checked_change(move |next| skip.revise(|value| *value = next)),
                )),
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Vertical orientation"),
                stepper("showcase-vertical")
                    .theme(theme.clone())
                    .name("Read-only progress")
                    .orientation(Orientation::Vertical)
                    .default_active(1)
                    .steps([
                        step("received")
                            .label("Received")
                            .description("Request received"),
                        step("processing")
                            .label("Processing")
                            .description("Waiting for confirmation"),
                        step("delivered")
                            .label("Delivered")
                            .description("Deliver the result"),
                    ]),
            ))
            .show_in(scope)
    })
}

fn tree_select_nodes() -> Vec<TreeNode> {
    vec![
        TreeNode::new("electronics", "Electronics").kind(TreeKind::Branch),
        TreeNode::new("phones", "Phones")
            .parent("electronics")
            .kind(TreeKind::Branch),
        TreeNode::new("iphone", "iPhone").parent("phones"),
        TreeNode::new("samsung", "Samsung Galaxy").parent("phones"),
        TreeNode::new("pixel", "Google Pixel").parent("phones"),
        TreeNode::new("laptops", "Laptops")
            .parent("electronics")
            .kind(TreeKind::Branch),
        TreeNode::new("macbook", "MacBook").parent("laptops"),
        TreeNode::new("thinkpad", "ThinkPad").parent("laptops"),
        TreeNode::new("headphones", "Headphones").parent("electronics"),
        TreeNode::new("groceries", "Groceries").kind(TreeKind::Branch),
        TreeNode::new("fruits", "Fruits")
            .parent("groceries")
            .kind(TreeKind::Branch),
        TreeNode::new("apples", "Apples").parent("fruits"),
        TreeNode::new("bananas", "Bananas").parent("fruits"),
        TreeNode::new("vegetables", "Vegetables")
            .parent("groceries")
            .kind(TreeKind::Branch),
        TreeNode::new("carrots", "Carrots").parent("vegetables"),
        TreeNode::new("broccoli", "Broccoli").parent("vegetables"),
        TreeNode::new("books", "Books"),
        TreeNode::new("toys", "Toys"),
    ]
}

fn tree_expanded() -> std::collections::BTreeSet<String> {
    [
        "electronics",
        "phones",
        "laptops",
        "groceries",
        "fruits",
        "vegetables",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn tree_choice(key: &str) -> TreeChoices {
    let model = TreeChoiceModel::new(
        TreeModel::new(tree_select_nodes()).expect("tree select example data"),
        TreeChoicePolicy::Leaves,
        false,
    )
    .expect("tree select example model");
    TreeChoices::new([model.path(key).expect("tree select example value")])
        .expect("tree select example choices")
}

fn removable_tree_choices(
    theme: Theme,
    values: Cage<TreeChoices>,
    policy: TreeChoicePolicy,
) -> impl Widget {
    let model = TreeChoiceModel::new(
        TreeModel::new(tree_select_nodes()).expect("tree select example data"),
        policy,
        true,
    )
    .expect("tree select example model");
    group()
        .theme(theme.clone())
        .gap(8.0)
        .wrap(FlexWrap::Wrap)
        .fill(
            each(values.map(|values| values.paths().to_vec()))
                .key(|path| path.leaf().to_owned())
                .children(move |entry| {
                    let path = entry.value.get();
                    let key = path.leaf().to_owned();
                    let label = model
                        .tree()
                        .index(&key)
                        .map(|index| model.tree().nodes()[index].label.clone())
                        .unwrap_or_default();
                    let item_model = model.clone();
                    pill()
                        .theme(theme.clone())
                        .content(label)
                        .removable(format!("Remove {key}"))
                        .on_remove(move || {
                            values.revise(|current| {
                                if let Ok(next) = item_model.toggle(current, &key) {
                                    *current = next;
                                }
                            })
                        })
                }),
        )
}

fn filter_tree_nodes(nodes: &[TreeNode], query: &str) -> Vec<TreeNode> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return nodes.to_vec();
    }
    nodes
        .iter()
        .filter(|candidate| {
            nodes.iter().any(|node| {
                if !node.label.to_lowercase().contains(&query) {
                    return false;
                }
                if node.key == candidate.key {
                    return true;
                }
                let mut parent = node.parent.as_deref();
                while let Some(key) = parent {
                    if key == candidate.key {
                        return true;
                    }
                    parent = nodes
                        .iter()
                        .find(|node| node.key == key)
                        .and_then(|node| node.parent.as_deref());
                }
                false
            })
        })
        .cloned()
        .collect()
}

fn tree_select_field(
    theme: &Theme,
    key: &'static str,
    label: &'static str,
    description: &'static str,
    value: Cage<TreeChoices>,
    multiple: bool,
    policy: TreeChoicePolicy,
) -> impl Widget {
    let class = format!("site-{key}");
    field(key)
        .theme(theme.clone())
        .label(label)
        .description(description)
        .control(move |field| {
            tree_select(key)
                .field(field)
                .class(class.clone())
                .nodes(tree_select_nodes())
                .value(value)
                .multiple(multiple)
                .policy(policy)
                .default_expanded(tree_expanded())
                .clearable(true)
                .placeholder(if multiple {
                    "Pick values"
                } else {
                    "Pick value"
                })
                .on_value_change(move |next| value.revise(|current| *current = next))
        })
}

fn documentation_heading(theme: &Theme, id: &'static str, title: &'static str) -> impl Widget {
    glory::ui::view()
        .semantics(Some(Box::new(glory::render::Semantics {
            id: Some(glory::render::SemanticId::new(id).expect("showcase section id")),
            ..Default::default()
        })))
        .fill(sapid::title(3).theme(theme.clone()).content(title))
}

fn calendar_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let provider = DatesProvider::new(date("2024-02-29"));
        let custom = scope.cage(DateSelection::Multiple(SelectedDates::new()));
        let selected = custom;
        let summary = custom.map(|selection| match selection {
            DateSelection::Multiple(values) => format!("Selected: {} / 3", values.len()),
            _ => unreachable!(),
        });
        let rendered_day = |date: Vary<CivilDate>| {
            widget(move |scope| {
                let date = date.clone();
                let value = scope.bond(move || {
                    let date = date.get();
                    if date.day() == 16 {
                        format!("{} •", date.day())
                    } else {
                        date.day().to_string()
                    }
                });
                glory::ui::text()
                    .class("site-calendar-rendered-day")
                    .content(value)
                    .show_in(scope)
            })
        };
        let children: Vec<Box<dyn Widget>> = vec![
            Box::new(documentation_heading(&theme, "calendar-usage", "Usage")),
            Box::new(
                calendar("calendar-usage-grid", provider)
                    .theme(theme.clone())
                    .name("Calendar usage")
                    .default_month(YearMonth::new(2024, 2).unwrap()),
            ),
            Box::new(documentation_heading(
                &theme,
                "calendar-custom-picker",
                "Custom date picker",
            )),
            Box::new(
                calendar("calendar-custom-grid", provider)
                    .theme(theme.clone())
                    .name("Pick up to three dates")
                    .default_month(YearMonth::new(2024, 2).unwrap())
                    .selection(custom)
                    .on_value_change(move |date| {
                        let Some(date) = date else { return };
                        selected.revise(|selection| {
                            let DateSelection::Multiple(mut values) = *selection else {
                                unreachable!()
                            };
                            if !values.remove(date) && values.len() < 3 {
                                values
                                    .insert(date)
                                    .expect("showcase selection stays bounded");
                            }
                            *selection = DateSelection::Multiple(values);
                        });
                    }),
            ),
            Box::new(sapid::text().theme(theme.clone()).content(summary)),
            Box::new(documentation_heading(
                &theme,
                "calendar-full-width",
                "Full width",
            )),
            Box::new(
                calendar("calendar-full-width-grid", provider)
                    .theme(theme.clone())
                    .name("Full width calendar")
                    .default_month(YearMonth::new(2024, 2).unwrap())
                    .full_width(true),
            ),
            Box::new(documentation_heading(
                &theme,
                "calendar-static",
                "Static calendar and custom days",
            )),
            Box::new(
                calendar("calendar-static-grid", provider)
                    .theme(theme.clone())
                    .name("Static event calendar")
                    .default_month(YearMonth::new(2024, 2).unwrap())
                    .static_mode(true)
                    .render_day(rendered_day),
            ),
            Box::new(documentation_heading(
                &theme,
                "calendar-options",
                "Week numbers and weekday header",
            )),
            Box::new(
                sapid::group().theme(theme.clone()).fill((
                    calendar("calendar-weeks-grid", provider)
                        .theme(theme.clone())
                        .name("Calendar with week numbers")
                        .default_month(YearMonth::new(2024, 2).unwrap())
                        .with_week_numbers(true),
                    calendar("calendar-no-weekdays-grid", provider)
                        .theme(theme.clone())
                        .name("Calendar without weekday header")
                        .default_month(YearMonth::new(2024, 2).unwrap())
                        .hide_weekdays(true),
                )),
            ),
        ];
        sapid::stack()
            .theme(theme.clone())
            .gap(24.0)
            .fill(children)
            .show_in(scope)
    })
}

fn tree_select_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let single = scope.cage(TreeChoices::default());
        let multiple = scope.cage(tree_choice("iphone"));
        let cascade = scope.cage(TreeChoices::default());
        let independent = scope.cage(TreeChoices::default());
        let searched = scope.cage(TreeChoices::default());
        let query = scope.cage(String::new());
        let controlled_expanded = scope.cage(tree_expanded());
        let node_theme = theme.clone();
        let value_theme = theme.clone();
        let children: Vec<Box<dyn Widget>> = vec![
            Box::new(documentation_heading(
                &theme,
                "tree-select-usage",
                "Usage and single selection",
            )),
            Box::new(tree_select_field(
                &theme,
                "tree-select-single",
                "Your favorite item",
                "Choose one item from the hierarchy.",
                single,
                false,
                TreeChoicePolicy::Leaves,
            )),
            Box::new(documentation_heading(
                &theme,
                "tree-select-modes",
                "Multiple and checkbox selection",
            )),
            Box::new(sapid::stack().theme(theme.clone()).gap(8.0).fill((
                tree_select_field(
                    &theme,
                    "tree-select-multiple",
                    "Your favorite items",
                    "Select several values and remove individual pills.",
                    multiple,
                    true,
                    TreeChoicePolicy::Leaves,
                ),
                removable_tree_choices(theme.clone(), multiple, TreeChoicePolicy::Leaves),
            ))),
            Box::new(
                sapid::title(4)
                    .theme(theme.clone())
                    .content("Checkbox selection with parent-child cascade"),
            ),
            Box::new(sapid::stack().theme(theme.clone()).gap(8.0).fill((
                tree_select_field(
                    &theme,
                    "tree-select-cascade",
                    "Include categories",
                    "Selecting a branch selects all of its enabled descendants.",
                    cascade,
                    true,
                    TreeChoicePolicy::DescendantLeaves,
                ),
                removable_tree_choices(theme.clone(), cascade, TreeChoicePolicy::DescendantLeaves),
            ))),
            Box::new(tree_select_field(
                &theme,
                "tree-select-independent",
                "Independent checkbox selection",
                "Check individual leaves without changing their parent or siblings.",
                independent,
                true,
                TreeChoicePolicy::AnyNode,
            )),
            Box::new(documentation_heading(
                &theme,
                "tree-select-searchable",
                "Searchable and nothing found",
            )),
            Box::new(
                field("tree-select-search")
                    .theme(theme.clone())
                    .label("Filtered items")
                    .description("Type in the control; matching items retain their ancestors.")
                    .control(move |field| {
                        tree_select("tree-select-search")
                            .field(field)
                            .class("site-tree-select-search")
                            .nodes(tree_select_nodes())
                            .value(searched)
                            .search(query)
                            .searchable(true)
                            .nothing_found_message("Nothing found...")
                            .default_expanded(tree_expanded())
                            .placeholder("Pick a matching item")
                            .on_value_change(move |next| searched.revise(|current| *current = next))
                            .on_search_change(move |next| query.revise(|current| *current = next))
                    }),
            ),
            Box::new(documentation_heading(
                &theme,
                "tree-select-expanded",
                "Controlled expansion and clearable",
            )),
            Box::new(
                field("tree-select-expansion")
                    .theme(theme.clone())
                    .label("Any node can be selected")
                    .description("Expand or collapse branches without losing the current value.")
                    .control(move |field| {
                        tree_select("tree-select-expansion")
                            .field(field)
                            .nodes(tree_select_nodes())
                            .policy(TreeChoicePolicy::AnyNode)
                            .expanded(controlled_expanded)
                            .clearable(true)
                            .on_expanded_change(move |next| {
                                controlled_expanded.revise(|current| *current = next)
                            })
                    }),
            ),
            Box::new(
                group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Expand all")
                        .on_press(move || {
                            controlled_expanded.revise(|current| {
                                *current = tree_expanded();
                            })
                        }),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Collapse all")
                        .on_press(move || controlled_expanded.revise(|current| current.clear())),
                )),
            ),
            Box::new(documentation_heading(
                &theme,
                "tree-select-limits",
                "Selection limit and hidden picked items",
            )),
            Box::new(
                tree_select("tree-select-limited")
                    .theme(theme.clone())
                    .name("Pick up to 2 items")
                    .nodes(tree_select_nodes())
                    .multiple(true)
                    .searchable(true)
                    .hide_picked(true)
                    .max_values(2)
                    .default_expanded(tree_expanded())
                    .placeholder("Pick values"),
            ),
            Box::new(documentation_heading(
                &theme,
                "tree-select-rendering",
                "Custom node and selected value rendering",
            )),
            Box::new(
                tree_select("tree-select-rendered")
                    .theme(theme.clone())
                    .name("Rendered categories")
                    .nodes(tree_select_nodes())
                    .multiple(true)
                    .default_value(tree_choice("headphones"))
                    .default_expanded(tree_expanded())
                    .render_node(move |node| {
                        group().theme(node_theme.clone()).gap(6.0).fill((
                            theme_icon(Icon::ChevronRight)
                                .theme(node_theme.clone())
                                .size(Size::ExtraSmall)
                                .variant(ThemeIconVariant::Soft),
                            text().content(node.with(|node| node.label.clone())),
                        ))
                    })
                    .render_value(move |node| {
                        badge()
                            .theme(value_theme.clone())
                            .content(node.with(|node| node.label.clone()))
                    }),
            ),
            Box::new(documentation_heading(
                &theme,
                "tree-select-dropdown",
                "Dropdown size, position and connecting lines",
            )),
            Box::new(
                tree_select("tree-select-dropdown")
                    .theme(theme.clone())
                    .name("Compact dropdown without lines")
                    .nodes(tree_select_nodes())
                    .default_expanded(tree_expanded())
                    .with_lines(false)
                    .popup_width(220.)
                    .max_dropdown_height(180.)
                    .popup_placement(glory::render::AnchorPlacement {
                        side: glory::render::AnchorSide::Top,
                        flip: false,
                        ..glory::render::AnchorPlacement::default()
                    }),
            ),
            Box::new(documentation_heading(
                &theme,
                "tree-select-states",
                "Disabled, read only and error",
            )),
            Box::new(
                tree_select("tree-select-disabled")
                    .theme(theme.clone())
                    .name("Disabled tree select")
                    .nodes(tree_select_nodes())
                    .default_value(tree_choice("headphones"))
                    .enabled(false),
            ),
            Box::new(
                tree_select("tree-select-readonly")
                    .theme(theme.clone())
                    .name("Read only tree select")
                    .nodes(tree_select_nodes())
                    .default_value(tree_choice("headphones"))
                    .readonly(true),
            ),
            Box::new(
                field("tree-select-error")
                    .theme(theme.clone())
                    .label("Favorite item")
                    .error("Choose an item before continuing")
                    .invalid(true)
                    .control(move |field| {
                        tree_select("tree-select-error")
                            .field(field)
                            .nodes(tree_select_nodes())
                            .placeholder("Pick value")
                            .invalid(true)
                    }),
            ),
        ];
        sapid::stack()
            .theme(theme.clone())
            .gap(24.0)
            .fill(children)
            .show_in(scope)
    })
}

fn framework_options() -> Vec<sapid::NativeOption> {
    ["React", "Angular", "Vue", "Svelte"]
        .into_iter()
        .map(|label| select_option(label.to_lowercase(), label))
        .collect()
}

fn autocomplete_options() -> Vec<sapid::NativeOption> {
    ["React", "Angular", "Vue", "Svelte"]
        .into_iter()
        .map(|label| autocomplete_option(label.to_lowercase(), label))
        .collect()
}

fn select_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<String>);
        field("select-framework")
            .theme(theme.clone())
            .label("Your favorite framework")
            .description("Choose one option. Keyboard typing moves through matching labels.")
            .control(move |field| {
                select("select-framework")
                    .field(field)
                    .value(value)
                    .options(framework_options())
                    .group(
                        select_group("Other libraries")
                            .option(select_option("solid", "Solid"))
                            .option(select_option("qwik", "Qwik")),
                    )
                    .clearable(true)
                    .placeholder("Pick value")
                    .on_value_change(move |next| value.revise(|current| *current = next))
            })
            .show_in(scope)
    })
}

fn combobox_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<String>);
        let search = scope.cage(String::new());
        stack()
            .theme(theme.clone())
            .gap(20.0)
            .fill((
                field("combobox-framework")
                    .theme(theme.clone())
                    .label("Your favorite framework")
                    .description("Type to filter options, then choose a suggestion.")
                    .control(move |field| {
                        combobox("combobox-framework")
                            .field(field)
                            .value(value)
                            .search(search)
                            .options(framework_options())
                            .empty_message("Nothing found")
                            .placeholder("Search or pick value")
                            .on_value_change(move |next| value.revise(|current| *current = next))
                            .on_search_change(move |next| search.revise(|current| *current = next))
                    }),
                field("combobox-prefix-limit")
                    .theme(theme.clone())
                    .label("Prefix filter with visible limit")
                    .description("Four declarations match A, but only two results are mounted.")
                    .control(move |field| {
                        combobox("combobox-prefix-limit")
                            .field(field)
                            .default_search("a")
                            .default_open(true)
                            .filter(ComboboxFilter::Prefix)
                            .limit(2)
                            .loading(true)
                            .loading_left(true)
                            .options([
                                select_option("angular", "Angular"),
                                select_option("astro", "Astro"),
                                select_option("alpine", "Alpine"),
                                select_option("ark", "Ark UI"),
                                select_option("react", "React"),
                            ])
                    }),
                field("combobox-groups")
                    .theme(theme.clone())
                    .label("Grouped options")
                    .description(
                        "Groups preserve declaration order and disabled options are skipped.",
                    )
                    .control(move |field| {
                        combobox("combobox-groups")
                            .field(field)
                            .group(select_group("Frontend").options([
                                select_option("vue", "Vue"),
                                select_option("svelte", "Svelte"),
                            ]))
                            .group(select_group("Backend").options([
                                select_option("axum", "Axum"),
                                select_option("django", "Django").disabled(true),
                            ]))
                            .placeholder("Search grouped options")
                    }),
            ))
            .show_in(scope)
    })
}

fn autocomplete_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(String::new());
        let selected = scope.cage(None::<String>);
        field("autocomplete-framework")
            .theme(theme.clone())
            .label("Your favorite framework")
            .description("Enter any text or pick one of the suggestions.")
            .control(move |field| {
                autocomplete("autocomplete-framework")
                    .field(field)
                    .value(value)
                    .selected_key(selected)
                    .options(autocomplete_options())
                    .empty_message("Nothing found")
                    .placeholder("Start typing")
                    .on_value_change(move |next| {
                        value.revise(|current| *current = next);
                        selected.revise(|current| *current = None);
                    })
                    .on_option_select(move |next| selected.revise(|current| *current = Some(next)))
            })
            .show_in(scope)
    })
}

fn cascader_demo(theme: Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(TreeChoices::default());
        let selected = value.map(|choices| {
            if choices.is_empty() {
                "No item selected".to_owned()
            } else {
                choices
                    .paths()
                    .iter()
                    .map(|path| path.keys().join(" / "))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(16.0)
            .fill((
                field("cascader-item")
                    .theme(theme.clone())
                    .label("Your favorite item")
                    .description("Move through successive columns to choose a nested value.")
                    .control(move |field| {
                        cascader("cascader-item")
                            .field(field)
                            .class("site-cascader-item")
                            .nodes(tree_select_nodes())
                            .value(value)
                            .default_expanded(std::collections::BTreeSet::from([
                                "electronics".to_owned()
                            ]))
                            .clearable(true)
                            .placeholder("Pick value")
                            .on_value_change(move |next| value.revise(|current| *current = next))
                    }),
                sapid::text().theme(theme.clone()).content(selected),
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Disabled and read only"),
                cascader("cascader-disabled")
                    .theme(theme.clone())
                    .name("Disabled cascader")
                    .nodes(tree_select_nodes())
                    .default_value(tree_choice("headphones"))
                    .enabled(false),
                cascader("cascader-readonly")
                    .theme(theme.clone())
                    .name("Read only cascader")
                    .nodes(tree_select_nodes())
                    .default_value(tree_choice("headphones"))
                    .readonly(true),
            ))
            .show_in(scope)
    })
}

pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
    match name {
        "Center" => Box::new(
            center()
                .theme(theme.clone())
                .inline(true)
                .height(Length::Px(140.0))
                .fill(tile(&theme, "Centered content")),
        ),
        "Container" => Box::new(
            container()
                .theme(theme.clone())
                .max_width(Limit::Px(420.0))
                .fill(tile(&theme, "Bounded container content")),
        ),
        "Space" => Box::new(
            stack().theme(theme.clone()).fill((
                tile(&theme, "Before horizontal space"),
                space().theme(theme.clone()).width(Length::Px(32.0)).height(Length::Px(24.0)),
                tile(&theme, "After horizontal space"),
            )),
        ),
        "Flex" => Box::new(
            flex()
                .theme(theme.clone())
                .direction(FlexDirection::Row)
                .gap(12.0)
                .fill((tile(&theme, "First"), tile(&theme, "Second"), tile(&theme, "Third"))),
        ),
        "Group" => Box::new(
            group()
                .theme(theme.clone())
                .gap(12.0)
                .wrap(FlexWrap::Wrap)
                .fill((tile(&theme, "One"), tile(&theme, "Two"), tile(&theme, "Three"))),
        ),
        "Stack" => Box::new(
            stack()
                .theme(theme.clone())
                .gap(12.0)
                .fill((tile(&theme, "First"), tile(&theme, "Second"), tile(&theme, "Third"))),
        ),
        "Box" => Box::new(tile(&theme, "A themed box with arbitrary child content")),
        "Grid" => Box::new(
            grid("showcase-grid")
                .theme(theme.clone())
                .columns(3)
                .gap(12.0)
                .items((1..=6).map(|index| {
                    grid_item(format!("showcase-grid-{index}"))
                        .theme(theme.clone())
                        .span(if index == 1 { 2 } else { 1 })
                        .fill(tile(&theme, &format!("Grid item {index}")))
                })),
        ),
        "SimpleGrid" => Box::new(
            simple_grid("showcase-simple-grid")
                .theme(theme.clone())
                .columns(3)
                .gap(12.0)
                .items((1..=6).map(|index| {
                    grid_item(format!("showcase-simple-grid-{index}"))
                        .theme(theme.clone())
                        .fill(tile(&theme, &format!("Item {index}")))
                })),
        ),
        "AlphaSlider" => Box::new(
            alpha_slider("showcase-alpha")
                .theme(theme)
                .name("Opacity")
                .default_value(0.65)
                .color(Color(34, 139, 230, 255)),
        ),
        "HueSlider" => Box::new(
            hue_slider("showcase-hue")
                .theme(theme)
                .name("Hue")
                .default_value(210.0),
        ),
        "ColorInput" => Box::new(widget(move |scope| {
            let hex = scope.cage("#228be6".to_owned());
            field("showcase-color-input").theme(theme.clone()).label("Input label").description("Input description").control(move |field| {
                    color_input("showcase-color-input").field(field).value(hex)
                        .on_value_change(move |next| hex.revise(|current| *current = next))
                }).show_in(scope)
        })),
        "ColorPicker" => Box::new(widget(move |scope| {
            let value = scope.cage(Hsva { hue: 210.0, saturation: 0.8, value: 0.9, alpha: 1.0 });
            color_picker("showcase-color-picker")
                .theme(theme.clone())
                .name("Accent color")
                .value(value)
                .show_precision_sliders(false)
                .on_value_change(move |next| value.revise(|current| *current = next))
                .show_in(scope)
        })),
        "FileInput" => Box::new(
            file_input()
                .theme(theme)
                .name("Attachments")
                .label("Choose files")
                .configuration(FileConfig { multiple: true, ..Default::default() }),
        ),
        "FileButton" => Box::new(
            file_button()
                .theme(theme)
                .label("Upload files")
                .configuration(FileConfig { multiple: true, ..Default::default() }),
        ),
        "ComboboxPopover" => Box::new(
            popover("showcase-combobox-popover")
                .theme(theme.clone())
                .label("Choose a framework")
                .title("Framework suggestions")
                .content(move || {
                    sapid::stack().theme(theme.clone()).gap(8.0).fill((
                        sapid::button().theme(theme.clone()).unstyled().label("Glory"),
                        sapid::button().theme(theme.clone()).unstyled().label("Sapid"),
                    ))
                }),
        ),
        "Popover" => Box::new(widget(move |scope| {
            let controlled = scope.cage(false);
            let controlled_copy = controlled;
            sapid::stack()
                .theme(theme.clone())
                .gap(18.0)
                .fill((
                    sapid::text().theme(theme.clone()).content("Basic usage"),
                    popover("showcase-popover-basic")
                        .theme(theme.clone())
                        .class("showcase-popover-basic")
                        .label("Toggle popover")
                        .title("Popover title")
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("Additional information stays beside its trigger.")
                        }),
                    sapid::text().theme(theme.clone()).content("Same width as target"),
                    popover("showcase-popover-target-width")
                        .theme(theme.clone())
                        .class("showcase-popover-target-width")
                        .label("Open matching panel")
                        .width(PopoverWidth::Target)
                        .trigger_content(|| text().content("A deliberately wide target control"))
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("This panel follows the live trigger width.")
                        }),
                    sapid::text().theme(theme.clone()).content("Arrow, offset and placement"),
                    popover("showcase-popover-arrow")
                        .theme(theme.clone())
                        .class("showcase-popover-arrow")
                        .label("Open positioned panel")
                        .placement(AnchorPlacement {
                            side: AnchorSide::Top,
                            align: AnchorAlign::End,
                            gap: 12.0,
                            arrow: Some(AnchorArrow {
                                size: 10.0,
                                radius: 2.0,
                                offset: 14.0,
                                position: AnchorArrowPosition::Side,
                            }),
                            ..AnchorPlacement::default()
                        })
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("The arrow follows the side selected after collision handling.")
                        }),
                    sapid::text().theme(theme.clone()).content("Controlled state"),
                    popover("showcase-popover-controlled")
                        .theme(theme.clone())
                        .class("showcase-popover-controlled")
                        .label("Toggle controlled popover")
                        .open(controlled)
                        .on_open_change(move |next| controlled_copy.revise(|value| *value = next))
                        .dismiss_on_outside(false)
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("Outside clicks are disabled; use the close action or Escape.")
                        }),
                ))
                .show_in(scope)
        })),
        "Pill" => Box::new(
            group().theme(theme.clone()).fill((
                pill().theme(theme.clone()).content("Rust"),
                pill().theme(theme.clone()).content("Removable").removable("Remove pill"),
                pill().theme(theme.clone()).content("Disabled").removable("Remove disabled pill").enabled(false),
            )),
        ),
        "Badge" => Box::new(
            group().theme(theme.clone()).fill((
                badge().theme(theme.clone()).content("Default"),
                badge().theme(theme.clone()).variant(BadgeVariant::Solid).content("Solid"),
                badge().theme(theme.clone()).variant(BadgeVariant::Outline).content("Outline"),
            )),
        ),
        "PillsInput" | "TagsInput" => {
            let pills = name == "PillsInput";
            Box::new(widget(move |scope| {
                let state = scope.cage(TagsInputState {
                    values: vec!["Rust".into(), "WebAssembly".into()],
                    search: String::new(),
                });
                field("showcase-token-input")
                    .theme(theme.clone())
                    .label(if pills { "Labels" } else { "Skills" })
                    .description("Press Enter to add a value. Backspace removes the last value when search is empty.")
                    .control(move |field| {
                        let input: Box<dyn Widget> = if pills {
                            Box::new(
                                pills_input("showcase-pills-input")
                                    .field(field)
                                    .state(state)
                                    .placeholder("Add a value")
                                    .on_change(move |next| state.revise(|current| *current = next)),
                            )
                        } else {
                            Box::new(
                                tags_input("showcase-tags-input")
                                    .field(field)
                                    .state(state)
                                    .placeholder("Add a value")
                                    .on_change(move |next| state.revise(|current| *current = next)),
                            )
                        };
                        input
                    })
                    .show_in(scope)
            }))
        }
        "ActionIcon" => Box::new(
            group().theme(theme.clone()).gap(24.0).fill([
                ("Filled", ActionIconVariant::Filled),
                ("Light", ActionIconVariant::Light),
                ("Outline", ActionIconVariant::Outline),
                ("Subtle", ActionIconVariant::Subtle),
                ("Default", ActionIconVariant::Default),
            ].into_iter().map(|(label, variant)| {
                sapid::stack().theme(theme.clone()).gap(8.0).fill((
                    action_icon(format!("{label} refresh"))
                        .theme(theme.clone())
                        .variant(variant)
                        .icon(glory::ui::text().content("↻").selectable(false)),
                    sapid::text().theme(theme.clone()).variant(TextVariant::Caption).content(label),
                ))
            }).collect::<Vec<_>>()),
        ),
        "CloseButton" => Box::new(
            group().theme(theme.clone()).fill([
                Size::ExtraSmall,
                Size::Small,
                Size::Medium,
                Size::Large,
                Size::ExtraLarge,
            ].into_iter().enumerate().map(|(index, size)| {
                close_button().theme(theme.clone()).size(size).name(format!("Close size {}", index + 1))
            }).collect::<Vec<_>>()),
        ),
        "Button" => Box::new(
            button().theme(theme).label("Filled button"),
        ),
        "UnstyledButton" => Box::new(
            button()
                .theme(theme)
                .unstyled()
                .label("Unstyled button")
                .style(Style::new().padding(Edges::all(0.0))),
        ),
        "Progress" | "RingProgress" | "SemiCircleProgress" => progress_example(name, theme),
        "ActionBar" => Box::new(widget(move |scope| {
            let count = scope.cage(2usize);
            action_bar("showcase-action-bar")
                .theme(theme.clone())
                .name("Selected item actions")
                .selected_count(count)
                .group(toolbar_group(&theme, "selected-actions"))
                .show_in(scope)
        })),
        "Toolbar" => Box::new(
            toolbar("showcase-toolbar")
                .theme(theme.clone())
                .name("Editor toolbar")
                .group(toolbar_group(&theme, "editor-actions")),
        ),
        "LoadingOverlay" => Box::new(widget(move |scope| {
            let loading = scope.cage(true);
            sapid::stack().theme(theme.clone()).gap(12.0).fill((
                loading_overlay(loading)
                    .theme(theme.clone())
                    .name("Editor")
                    .label("Loading editor")
                    .fill(sapid::text_field().theme(theme.clone()).name("Document title").default_value("Quarterly report")),
                sapid::button().theme(theme.clone()).label(loading.map(|value| if *value { "Finish loading".to_owned() } else { "Start loading".to_owned() }))
                    .on_press(move || loading.revise(|current| *current = !*current)),
            )).show_in(scope)
        })),
        "Overlay" => Box::new(
            overlay()
                .theme(theme.clone())
                .color(theme.accent)
                .opacity(0.2)
                .fill(sapid::button().theme(theme).label("Content below overlay")),
        ),
        "Avatar" => Box::new(
            sapid::stack().theme(theme.clone()).gap(18.0).fill((
                group().theme(theme.clone()).gap(16.0).fill((
                    avatar("Ada Lovelace").theme(theme.clone()).fallback("AL")
                        .source(media("image-pattern.png")),
                    avatar("John Doe").theme(theme.clone()).fallback("JD"),
                    avatar("Square avatar").theme(theme.clone()).fallback("SQ")
                        .shape(AvatarShape::Square).size(Size::Large),
                    avatar("Large avatar").theme(theme.clone()).fallback("LG")
                        .size(Size::ExtraLarge),
                )),
                sapid::title(3).theme(theme.clone()).content("Avatar.Group"),
                avatar_group("showcase-team")
                    .theme(theme.clone())
                    .name("Project team")
                    .max_visible(2)
                    .avatar("ada", avatar("Ada Lovelace").fallback("AL"))
                    .avatar("grace", avatar("Grace Hopper").fallback("GH"))
                    .avatar("katherine", avatar("Katherine Johnson").fallback("KJ"))
                    .avatar("margaret", avatar("Margaret Hamilton").fallback("MH")),
            )),
        ),
        "BackgroundImage" => Box::new(
            sapid::stack().theme(theme.clone()).gap(12.0).fill((
                sapid::text().theme(theme.clone()).content("Usage"),
                background_image(media("image-pattern.png"))
                    .theme(theme.clone())
                    .class("showcase-background-image")
                    .name("Pattern background")
                    .width(Length::Px(360.0))
                    .height(Length::Px(180.0))
                    .fill(sapid::text().theme(theme).content("Content over a background image")),
            )),
        ),
        "Image" => {
            let pattern = media("image-pattern.png");
            let fallback = media("image-fallback.png");
            Box::new(sapid::stack().theme(theme.clone()).gap(14.0).fill((
                sapid::text().theme(theme.clone()).content("Usage"),
                image(pattern.clone())
                    .theme(theme.clone())
                    .class("showcase-image-usage")
                    .alt("Abstract color pattern")
                    .width(Length::Px(320.0)),
                sapid::text().theme(theme.clone()).content("Image height"),
                image(pattern.clone())
                    .theme(theme.clone())
                    .class("showcase-image-height")
                    .alt("Fixed height color pattern")
                    .width(Length::Px(320.0))
                    .height(Length::Px(160.0)),
                sapid::text().theme(theme.clone()).content("Image fit"),
                image(pattern.clone())
                    .theme(theme.clone())
                    .class("showcase-image-fit")
                    .alt("Contained color pattern")
                    .width(Length::Px(320.0))
                    .height(Length::Px(160.0))
                    .fit(ObjectFit::Contain),
                sapid::text().theme(theme.clone()).content("Fallback image"),
                image(media("image-invalid.png"))
                    .theme(theme.clone())
                    .class("showcase-image-fallback")
                    .alt("Fallback color pattern")
                    .fallback_src(fallback.clone())
                    .width(Length::Px(320.0))
                    .height(Length::Px(160.0))
                    .fit(ObjectFit::Cover),
                sapid::text().theme(theme.clone()).content("Responsive source and loading"),
                image(pattern.clone())
                    .theme(theme)
                    .class("showcase-image-responsive")
                    .alt("Responsive color pattern")
                    .srcset([
                        ImageCandidate { source: pattern, width: 320 },
                        ImageCandidate { source: fallback, width: 960 },
                    ])
                    .sizes([
                        ImageSizeHint {
                            max_viewport_width: Some(600),
                            slot: ImageSlotSize::ViewportWidth(100),
                        },
                        ImageSizeHint {
                            max_viewport_width: None,
                            slot: ImageSlotSize::Pixels(480),
                        },
                    ])
                    .loading(ImageLoading::Lazy)
                    .width(Length::Px(480.0))
                    .height(Length::Px(180.0))
                    .fit(ObjectFit::Cover),
            )))
        }
        "Card" => Box::new(
            card()
                .theme(theme.clone())
                .title("Norway Fjord Adventures")
                .description("Explore the magical fjord landscapes with guided tours and outdoor activities.")
                .header(
                    image(media("image-pattern.png"))
                        .theme(theme.clone())
                        .alt("Abstract mountain and fjord landscape")
                        .width(Length::Percent(100.0))
                        .height(Length::Px(160.0))
                        .fit(ObjectFit::Cover),
                )
                .fill(
                    sapid::stack().theme(theme.clone()).gap(12.0).fill((
                        sapid::badge().theme(theme.clone()).content("On sale"),
                        sapid::text().theme(theme.clone()).content("Book a small-group tour with a local guide, travel insurance and flexible cancellation."),
                    )),
                )
                .footer(
                    group().theme(theme.clone()).fill((
                        sapid::button().theme(theme.clone()).label("Book tour"),
                        sapid::button().theme(theme).variant(sapid::ButtonVariant::Secondary).label("View details"),
                    )),
                ),
        ),
        "Paper" => Box::new(
            group().theme(theme.clone()).gap(16.0).wrap(FlexWrap::Wrap).fill([
                (ShadowSize::None, "Flat paper"),
                (ShadowSize::Small, "Small shadow"),
                (ShadowSize::Large, "Large shadow"),
            ].into_iter().map(|(shadow, label)| {
                paper()
                    .theme(theme.clone())
                    .bordered(shadow != ShadowSize::None)
                    .shadow(shadow)
                    .style(Style::new().padding(Edges::all(20.0)).radius(theme.radius)
                        .width(Length::Px(180.0)))
                    .fill(sapid::text().theme(theme.clone()).content(label))
            }).collect::<Vec<_>>()),
        ),
        "Indicator" => Box::new(
            indicator()
                .theme(theme.clone())
                .label("3")
                .name("Unread messages")
                .fill(sapid::button().theme(theme).label("Inbox")),
        ),
        "ThemeIcon" => Box::new(
            group().theme(theme.clone()).fill((
                theme_icon(Icon::Check).theme(theme.clone()).name("Completed"),
                theme_icon(Icon::Info).theme(theme.clone()).name("Information").variant(ThemeIconVariant::Outline),
                theme_icon(Icon::Close).theme(theme).name("Close").variant(ThemeIconVariant::Solid),
            )),
        ),
        "RollingNumber" => Box::new(
            rolling_number()
                .theme(theme)
                .value(1234.25)
                .min_fraction_digits(2)
                .max_fraction_digits(2)
                .duration_millis(800u16),
        ),
        "Marquee" => Box::new(
            marquee()
                .theme(theme)
                .text("Latest updates: native and browser motion preserve stable accessible text")
                .period_millis(1200u32)
                .style(Style::new().width(Length::Px(320.0)).max_width(Limit::Percent(100.0))),
        ),
        "List" => Box::new(
            sapid::stack().theme(theme.clone()).gap(20.0).fill((
                text_list().theme(theme.clone()).name("With icons")
                    .item(text_list_item().fill((text().content("✓ "), text().content("Clone or download the repository"))))
                    .item(text_list_item().fill((text().content("✓ "), text().content("Install dependencies"))))
                    .item(text_list_item().fill((text().content("✓ "), text().content("Run the development server")))),
                text_list().theme(theme.clone()).ordered(true).name("Nested list")
                    .item(text_list_item().fill(text().content("Create the application")))
                    .item(text_list_item().fill((
                        text().content("Compose widgets"),
                        text_list().theme(theme.clone()).ordered(true)
                            .item(text_list_item().fill(text().content("Keep keys stable")))
                            .item(text_list_item().fill((
                                text().content("Nest one more level"),
                                text_list().theme(theme.clone())
                                    .item(text_list_item().fill(text().content("Use semantic controls"))),
                            ))),
                    )))
                    .item(text_list_item().fill(text().content("Choose a host"))),
            )),
        ),
        "Code" => Box::new(
            sapid::stack().theme(theme.clone()).gap(16.0).fill((
                sapid::text().theme(theme.clone()).content("Inline code: let answer = 42;"),
                code().theme(theme.clone()).content("let answer = 42;"),
                code_block().theme(theme.clone()).source("fn main() {\n    println!(\"Hello\");\n}"),
                group().theme(theme.clone()).fill((
                    code().theme(theme.clone()).content("Accent code").style(Style {
                        background: Some(theme.accent),
                        color: Some(theme.on_accent),
                        ..Style::new().padding(Edges::all(4.0)).radius(theme.radius)
                    }),
                    code().theme(theme.clone()).content("Soft accent code").style(Style {
                        background: Some(theme.accent_soft),
                        color: Some(theme.accent_text),
                        ..Style::new().padding(Edges::all(4.0)).radius(theme.radius)
                    }),
                )),
            )),
        ),
        "CodeHighlight" => Box::new(code_highlight_demo(theme)),
        "Highlight" => Box::new(
            highlight()
                .theme(theme)
                .content("Highlight only the important word in this sentence.")
                .query("important"),
        ),
        "Mark" => Box::new(mark().theme(theme).content("Marked text")),
        "Stepper" => Box::new(stepper_demo(theme)),
        "Table" => Box::new(table_demo(theme)),
        "Select" => Box::new(select_demo(theme)),
        "Combobox" => Box::new(combobox_demo(theme)),
        "Autocomplete" => Box::new(autocomplete_demo(theme)),
        "Cascader" => Box::new(cascader_demo(theme)),
        "TreeSelect" => Box::new(tree_select_demo(theme)),
        "Text" => Box::new(
            sapid::stack().theme(theme.clone()).gap(14.0).fill((
                sapid::text().theme(theme.clone()).content("Usage"),
                sapid::text().theme(theme.clone()).content("Default text"),
                sapid::text().theme(theme.clone()).variant(TextVariant::Lead).content("Lead text"),
                sapid::text().theme(theme.clone()).variant(TextVariant::Muted).content("Muted text"),
                sapid::text().theme(theme.clone()).content("Single-line truncate"),
                sapid::text()
                    .theme(theme.clone())
                    .class("showcase-text-truncate")
                    .truncate(true)
                    .style(Style::new().width(Length::Px(320.0)))
                    .content("This sentence is deliberately too long for the available single line width."),
                sapid::text().theme(theme.clone()).content("Line clamp"),
                sapid::text()
                    .theme(theme.clone())
                    .class("showcase-text-clamp")
                    .line_clamp(2)
                    .style(Style::new().width(Length::Px(340.0)))
                    .content("This paragraph demonstrates a real two-line clamp. The complete value remains in the text node for copying and accessibility even though additional visual lines are clipped by the host."),
                sapid::text().theme(theme.clone()).content("Gradient"),
                sapid::text()
                    .theme(theme)
                    .class("showcase-text-gradient")
                    .font_size(30.0)
                    .weight(700.0)
                    .gradient(glory::style::LinearGradient {
                        angle: 90.0,
                        stops: vec![Color(30, 110, 240, 255), Color(190, 40, 210, 255)],
                    })
                    .content("Gradient text"),
            )),
        ),
        "Title" => Box::new(
            sapid::stack().theme(theme.clone()).gap(12.0).fill((
                title(1).theme(theme.clone()).content("Heading level 1"),
                title(2).theme(theme.clone()).content("Heading level 2"),
                title(3).theme(theme.clone()).content("Heading level 3"),
                sapid::text().theme(theme.clone()).content("Line clamp"),
                title(2)
                    .theme(theme)
                    .class("showcase-title-clamp")
                    .line_clamp(2)
                    .style(Style::new().width(Length::Px(360.0)))
                    .content("A heading can be constrained to two lines while retaining its complete semantic name and heading level."),
            )),
        ),
        "Typography" => Box::new(
            typography().theme(theme).content(|context| (
                context.title(2).content("Rich typography"),
                context.text().content("Text and headings inherit the same theme and size."),
                context.anchor().label("Read the documentation").href("https://sapid.taidge.org/"),
            )),
        ),
        "FocusTrap" => Box::new(widget(move |scope| {
            let active = scope.cage(true);
            sapid::stack().theme(theme.clone()).gap(12.0).fill((
                focus_scope()
                    .active(active)
                    .fill((
                        sapid::text_field().theme(theme.clone()).name("First field"),
                        sapid::text_field().theme(theme.clone()).name("Second field"),
                    )),
                sapid::button().theme(theme.clone()).label(active.map(|value| if *value { "Deactivate focus trap".to_owned() } else { "Activate focus trap".to_owned() }))
                    .on_press(move || active.revise(|current| *current = !*current)),
            )).show_in(scope)
        })),
        "Portal" => Box::new(widget(move |scope| {
            let context = portal_context(scope, "showcase-portal")?;
            let content_theme = theme.clone();
            view().fill((
                portal_target(&context, "destination").fill(tile(&theme, "Portal destination")),
                portal(&context, "destination").content(move || {
                    sapid::text().theme(content_theme.clone()).content("This content is rendered in the portal destination.")
                }),
            )).show_in(scope)
        })),
        "ScrollArea" | "Scroller" => {
            let horizontal = name == "Scroller";
            Box::new(widget(move |scope| {
                let content_theme = theme.clone();
                let content = move |_| {
                    let count = if horizontal { 16 } else { 24 };
                    let items = (1..=count).map(|index| tile(&content_theme, &format!("Item {index}"))).collect::<Vec<_>>();
                    if horizontal {
                        Box::new(group().theme(content_theme.clone()).wrap(FlexWrap::NoWrap).fill(items)) as Box<dyn Widget>
                    } else {
                        Box::new(sapid::stack().theme(content_theme.clone()).fill(items))
                    }
                };
                let area: Box<dyn Widget> = if horizontal {
                    Box::new(scroller("showcase-scroller").area(|area| area.theme(theme.clone())).content(content))
                } else {
                    Box::new(
                        scroll_area("showcase-scroll-area")
                            .theme(theme.clone())
                            .name("Scrollable content")
                            .content(content),
                    )
                };
                view().style(Style::new().height(Length::Px(220.0)).max_width(Limit::Px(560.0))).fill(area).show_in(scope)
            }))
        }
        "Notification" => Box::new(
            notification("showcase-notification")
                .theme(theme)
                .title("Changes saved")
                .content("Your changes are stored on this device.")
                .action("Undo", || {}),
        ),
        "Notifications system" => Box::new(widget(move |scope| {
            let handle = Rc::new(RefCell::new(None::<NotificationsHandle>));
            let target = handle.clone();
            let clock = NotificationClock::new(|| Duration::ZERO, |_| std::future::pending());
            sapid::stack().theme(theme.clone()).gap(12.0).fill((
                sapid::button().theme(theme.clone()).label("Show notification").on_press(move || {
                    if let Some(handle) = target.borrow().clone() {
                        let _ = handle.show(notification_request("saved", "Changes saved", "Your changes are stored on this device."));
                    }
                }),
                toaster("showcase-notifications")
                    .theme(theme.clone())
                    .capacity(8)
                    .visible_limit(4)
                    .clock(clock)
                    .on_ready(move |value| *handle.borrow_mut() = Some(value)),
            )).show_in(scope)
        })),
        "DatePicker" => Box::new(
            date_picker("showcase-date-picker", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Appointment date")
                .default_value(DateSelection::Single(Some(date("2024-02-29"))))
                .presets([
                    DatePickerPreset::new(
                        "Yesterday",
                        DateSelection::Single(Some(date("2024-02-28"))),
                    ),
                    DatePickerPreset::new(
                        "Today",
                        DateSelection::Single(Some(date("2024-02-29"))),
                    ),
                ]),
        ),
        "DatePickerInput" => Box::new(
            date_picker_input("showcase-date-picker-input", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Appointment date")
                .default_value(DateSelection::Single(Some(date("2024-02-29"))))
                .trigger_label("Choose appointment date")
                .presets([
                    DatePickerPreset::new(
                        "Yesterday",
                        DateSelection::Single(Some(date("2024-02-28"))),
                    ),
                    DatePickerPreset::new(
                        "Today",
                        DateSelection::Single(Some(date("2024-02-29"))),
                    ),
                ]),
        ),
        "DateTimePicker" => Box::new(
            date_time_picker("showcase-date-time", DatesProvider::new(date("2024-02-29")), DateTimePolicy::Civil)
                .theme(theme)
                .name("Appointment date and time")
                .default_value(Some(datetime("2024-02-29T10:20"))),
        ),
        "InlineDateTimePicker" => Box::new(
            inline_date_time_picker("showcase-inline-date-time", DatesProvider::new(date("2024-02-29")), DateTimePolicy::Civil)
                .theme(theme)
                .name("Inline appointment")
                .default_value(Some(datetime("2024-02-29T10:20"))),
        ),
        "MonthPicker" => Box::new(
            month_picker("showcase-month", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Billing month")
                .default_value(MonthSelection::Single(Some(YearMonth::new(2024, 2).unwrap()))),
        ),
        "MonthPickerInput" => Box::new(
            month_picker_input("showcase-month-input", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Billing month")
                .default_value(MonthSelection::Single(Some(YearMonth::new(2024, 2).unwrap()))),
        ),
        "YearPicker" => Box::new(
            year_picker("showcase-year", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Billing year")
                .default_value(YearSelection::Single(Some(CivilYear::new(2024).unwrap()))),
        ),
        "YearPickerInput" => Box::new(
            year_picker_input("showcase-year-input", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Billing year")
                .default_value(YearSelection::Single(Some(CivilYear::new(2024).unwrap()))),
        ),
        "TimeInput" => Box::new(
            time_input("showcase-time-input")
                .theme(theme)
                .name("Appointment time")
                .default_value(Some(LocalTime::new(10, 30, 0).unwrap())),
        ),
        "TimeValue" => Box::new(
            time_value(Some(LocalTime::new(10, 30, 0).unwrap()))
                .theme(theme)
                .empty_label("No time selected"),
        ),
        "TimePicker" => Box::new(
            time_picker("showcase-time-picker")
                .theme(theme)
                .name("Appointment time")
                .default_value(Some(LocalTime::new(10, 30, 0).unwrap())),
        ),
        "TimeGrid" => Box::new(
            time_grid("showcase-time-grid")
                .theme(theme)
                .name("Available times")
                .default_value(Some(LocalTime::new(10, 30, 0).unwrap())),
        ),
        "MiniCalendar" => Box::new(
            mini_calendar("showcase-mini-calendar", DatesProvider::new(date("2024-02-29")))
                .theme(theme)
                .name("Compact calendar")
                .default_value(Some(date("2024-02-29"))),
        ),
        "Calendar" => Box::new(
            calendar_demo(theme),
        ),
        "Schedule" => Box::new(
            schedule("showcase-schedule", schedule_data(ScheduleWindow::days(date("2024-02-29"), 1).unwrap()))
                .theme(theme)
                .name("Schedule")
                .viewport_height(360.0),
        ),
        "AgendaView" => Box::new(
            agenda_view("showcase-agenda", schedule_data(ScheduleWindow::days(date("2024-02-26"), 7).unwrap()))
                .theme(theme)
                .name("Upcoming events"),
        ),
        "DayView" => Box::new(
            day_view("showcase-day", schedule_data(ScheduleWindow::days(date("2024-02-29"), 1).unwrap()))
                .theme(theme)
                .name("Day schedule")
                .viewport_height(400.0),
        ),
        "WeekView" => Box::new(
            week_view("showcase-week", schedule_data(ScheduleWindow::days(date("2024-02-26"), 7).unwrap()))
                .theme(theme)
                .name("Week schedule")
                .viewport_height(400.0),
        ),
        "MonthView" => Box::new(
            month_view("showcase-month-view", schedule_data(ScheduleWindow::month(YearMonth::new(2024, 2).unwrap()).unwrap()))
                .theme(theme)
                .name("Month schedule"),
        ),
        "YearView" => Box::new(
            year_view("showcase-year-view", schedule_data(ScheduleWindow::year(CivilYear::new(2024).unwrap()).unwrap()))
                .theme(theme)
                .name("Year schedule"),
        ),
        "MobileMonthView" => Box::new(
            mobile_month_view("showcase-mobile-month", schedule_data(ScheduleWindow::month(YearMonth::new(2024, 2).unwrap()).unwrap()))
                .theme(theme)
                .name("Mobile month schedule"),
        ),
        "ResourcesDayView" | "ResourcesWeekView" | "ResourcesMonthView" | "ResourcesSchedule" => {
            let window = if name == "ResourcesMonthView" {
                ScheduleWindow::month(YearMonth::new(2024, 2).unwrap()).unwrap()
            } else if name == "ResourcesWeekView" {
                ScheduleWindow::days(date("2024-02-26"), 7).unwrap()
            } else {
                ScheduleWindow::days(date("2024-02-29"), 1).unwrap()
            };
            let data = resource_data(window);
            let widget: Box<dyn Widget> = match name {
                "ResourcesWeekView" => Box::new(resources_week_view("showcase-resource-week", data).theme(theme).name("Resources this week").viewport_height(400.0)),
                "ResourcesMonthView" => Box::new(resources_month_view("showcase-resource-month", data).theme(theme).name("Resources this month")),
                "ResourcesSchedule" => Box::new(resources_schedule("showcase-resources", data).theme(theme).name("Resource schedule").viewport_height(400.0)),
                _ => Box::new(resources_day_view("showcase-resource-day", data).theme(theme).name("Resources today").viewport_height(400.0)),
            };
            widget
        }
        "Recurring events" => Box::new(
            data_list()
                .theme(theme)
                .name("Recurrence rule")
                .item(data_list_item().term("Frequency").fill(text().content("Daily")))
                .item(data_list_item().term("Interval").fill(text().content("Every day")))
                .item(data_list_item().term("End").fill(text().content("Never"))),
        ),
        "Events data" => Box::new(
            data_list()
                .theme(theme)
                .name("Event data")
                .item(data_list_item().term("Time zone model").fill(text().content("Explicit civil dates and times")))
                .item(data_list_item().term("Window").fill(text().content("Finite, application supplied")))
                .item(data_list_item().term("Resources").fill(text().content("Optional stable resource keys"))),
        ),
        _ => unreachable!("unsupported named demo: {name}"),
    }
}
