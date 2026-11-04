use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("TreeSelect", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use glory::render::{FileConfig, Orientation};
    use glory::style::{FlexWrap, ObjectFit};
    use glory::ui::text;
    use sapid::prelude::*;
    use sapid::{
        Icon, ThemeIconVariant, autocomplete_option, background_image, button, file_button,
        file_input, group, image, indicator, progress, ring_progress, select_group, select_option,
        semi_circle_progress, theme_icon,
    };
    use std::{boxed::Box, cell::RefCell, rc::Rc, time::Duration};

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
                    removable_tree_choices(
                        theme.clone(),
                        cascade,
                        TreeChoicePolicy::DescendantLeaves,
                    ),
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
                                .on_value_change(move |next| {
                                    searched.revise(|current| *current = next)
                                })
                                .on_search_change(move |next| {
                                    query.revise(|current| *current = next)
                                })
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
                        .description(
                            "Expand or collapse branches without losing the current value.",
                        )
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
                            .on_press(move || {
                                controlled_expanded.revise(|current| current.clear())
                            }),
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

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "TreeSelect" => Box::new(tree_select_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
