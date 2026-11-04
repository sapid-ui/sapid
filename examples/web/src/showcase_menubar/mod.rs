use super::showcase_examples::DetailSection;
mod choices;
mod composition;
mod controlled;
mod keyboard;
mod position;
mod submenus;
mod trigger;

pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Composition", "#menubar-composition"),
        ("Trigger and loop", "#menubar-trigger"),
        ("Submenus", "#menubar-submenus"),
        ("Checkbox and radio", "#menubar-choices"),
        ("Controlled", "#menubar-controlled"),
        ("Position", "#menubar-position"),
        ("Keyboard", "#menubar-keyboard"),
    ]
}

pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "menubar-composition",
            title: "How it differs from Menu and composition",
            note: "Menu has one trigger. Menubar coordinates a row of menus with one tab stop and one open menu. Compose each root with menubar_menu, then add items, separators, radio groups and submenus.",
            example: Box::new(composition::demo(theme.clone())),
            source: include_str!("composition.rs"),
        },
        DetailSection {
            id: "menubar-trigger",
            title: "Trigger and loop",
            note: "Click opens a menu; moving over another target switches the open menu. open_on_hover(true) also opens a closed bar on mouse or pen movement and closes it when the pointer leaves the bar and its menus. loop_focus(false) stops arrows at either end. Touch uses activation.",
            example: Box::new(trigger::demo(theme.clone())),
            source: include_str!("trigger.rs"),
        },
        DetailSection {
            id: "menubar-submenus",
            title: "Submenus",
            note: "Compose nested commands with menu_submenu. ArrowRight enters a submenu, ArrowLeft returns to its parent; the directions reverse in RTL. Escape closes a menu and restores its trigger.",
            example: Box::new(submenus::demo(theme.clone())),
            source: include_str!("submenus.rs"),
        },
        DetailSection {
            id: "menubar-choices",
            title: "Checkbox and radio items",
            note: "menu_checkbox toggles a boolean; menu_radio_group selects one keyed value. Both keep the menu open by default. close_on_select(true) changes that policy. Use local defaults or controlled Cage/Bond/Vary sources.",
            example: Box::new(choices::demo(theme.clone())),
            source: include_str!("choices.rs"),
        },
        DetailSection {
            id: "menubar-controlled",
            title: "Controlled",
            note: "value and on_value_change control the opened root by its stable key; None closes all menus. default_value chooses an initial menu for local state. Rejected proposals preserve the accepted menu and focus.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "menubar-position",
            title: "Position and dropdown width",
            note: "position sets an AnchorPlacement for the bar. Each menubar_menu can override position and width independently. The shared host layer handles flip and shift. Width is bounded to 80–1000 CSS pixels.",
            example: Box::new(position::demo(theme.clone())),
            source: include_str!("position.rs"),
        },
        DetailSection {
            id: "menubar-keyboard",
            title: "Keyboard interactions and accessibility",
            note: "Tab enters or leaves one tab stop. Left/Right move between roots; Home/End choose the first/last enabled root. Down/Enter/Space open the first item, Up opens the last. Characters match labels, disabled roots are skipped, and Escape restores the trigger. The bar exports menubar/menuitem roles, popup and expanded state. RTL reverses horizontal arrows.",
            example: Box::new(keyboard::demo(theme)),
            source: include_str!("keyboard.rs"),
        },
    ]
}
