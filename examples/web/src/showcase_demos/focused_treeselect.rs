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
            TreeNode::new("phones", "Phones").parent("electronics"),
            TreeNode::new("laptops", "Laptops").parent("electronics"),
            TreeNode::new("headphones", "Headphones").parent("electronics"),
            TreeNode::new("groceries", "Groceries").kind(TreeKind::Branch),
            TreeNode::new("fruit", "Fruit").parent("groceries"),
            TreeNode::new("vegetables", "Vegetables").parent("groceries"),
            TreeNode::new("books", "Books"),
            TreeNode::new("toys", "Toys").disabled(true),
        ]
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
                    .default_expanded(std::collections::BTreeSet::from([
                        "electronics".to_owned(),
                        "groceries".to_owned(),
                    ]))
                    .clearable(true)
                    .placeholder(if multiple {
                        "Pick values"
                    } else {
                        "Pick value"
                    })
                    .on_value_change(move |next| value.revise(|current| *current = next))
            })
    }

    fn tree_select_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let single = scope.cage(TreeChoices::default());
            let multiple = scope.cage(tree_choice("phones"));
            let cascade = scope.cage(TreeChoices::default());
            let independent = scope.cage(TreeChoices::default());
            let searched = scope.cage(TreeChoices::default());
            let query = scope.cage(String::new());
            let expanded = scope.cage(std::collections::BTreeSet::from([
                "electronics".to_owned(),
                "groceries".to_owned(),
            ]));
            let filtered =
                scope.bond(move || filter_tree_nodes(&tree_select_nodes(), &query.get()));
            let nothing_found = scope.bond(move || filtered.with(Vec::is_empty));
            let controlled_expanded = scope.cage(std::collections::BTreeSet::from([
                "electronics".to_owned(),
                "groceries".to_owned(),
            ]));
            let children: Vec<Box<dyn Widget>> = vec![
            Box::new(tree_select_field(
                &theme,
                "tree-select-single",
                "Your favorite item",
                "Choose one item from the hierarchy.",
                single,
                false,
                TreeChoicePolicy::Leaves,
            )),
            Box::new(
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Multiple selection"),
            ),
            Box::new(
                sapid::stack()
                    .theme(theme.clone())
                    .gap(8.0)
                    .fill((
                        tree_select_field(
                            &theme,
                            "tree-select-multiple",
                            "Your favorite items",
                            "Select several values and remove individual pills.",
                            multiple,
                            true,
                            TreeChoicePolicy::Leaves,
                        ),
                        removable_tree_choices(
                            theme.clone(),
                            multiple,
                            TreeChoicePolicy::Leaves,
                        ),
                    )),
            ),
            Box::new(
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Checkbox selection with parent-child cascade"),
            ),
            Box::new(
                sapid::stack()
                    .theme(theme.clone())
                    .gap(8.0)
                    .fill((
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
                    )),
            ),
            Box::new(tree_select_field(
                &theme,
                "tree-select-independent",
                "Independent checkbox selection",
                "Check individual leaves without changing their parent or siblings.",
                independent,
                true,
                TreeChoicePolicy::Leaves,
            )),
            Box::new(
                sapid::title(3).theme(theme.clone()).content("Search"),
            ),
            Box::new(
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Search tree items")
                    .placeholder("Search items")
                    .value(query)
                    .on_value_change(move |next| {
                        query.revise(|current| *current = next);
                        searched.revise(|current| *current = TreeChoices::default());
                        expanded.revise(|current| {
                            *current = std::collections::BTreeSet::from([
                                "electronics".to_owned(),
                                "groceries".to_owned(),
                            ]);
                        });
                    }),
            ),
            Box::new(
                field("tree-select-search")
                    .theme(theme.clone())
                    .label("Filtered items")
                    .description("Matching items retain their ancestors. Disabled items remain unavailable.")
                    .control(move |field| {
                        tree_select("tree-select-search")
                            .field(field)
                            .class("site-tree-select-search")
                            .nodes(filtered)
                            .value(searched)
                            .expanded(expanded)
                            .placeholder("Pick a matching item")
                            .on_value_change(move |next| {
                                searched.revise(|current| *current = next)
                            })
                            .on_expanded_change(move |next| {
                                expanded.revise(|current| *current = next)
                            })
                    }),
            ),
            Box::new(when(nothing_found, || {
                text()
                    .class("site-tree-select-empty")
                    .content("Nothing found")
            })),
            Box::new(
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Controlled expansion"),
            ),
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
            Box::new(group().theme(theme.clone()).fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Expand all")
                    .on_press(move || {
                        controlled_expanded.revise(|current| {
                            *current = std::collections::BTreeSet::from([
                                "electronics".to_owned(),
                                "groceries".to_owned(),
                            ]);
                        })
                    }),
                sapid::button()
                    .theme(theme.clone())
                    .label("Collapse all")
                    .on_press(move || controlled_expanded.revise(|current| current.clear())),
            ))),
            Box::new(
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Disabled, read only and error"),
            ),
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
