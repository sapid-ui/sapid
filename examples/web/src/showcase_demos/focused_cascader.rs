use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Cascader", theme)
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
                                .on_value_change(move |next| {
                                    value.revise(|current| *current = next)
                                })
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
            "Cascader" => Box::new(cascader_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
