use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::breadcrumbs("custom-separator")
        .theme(theme)
        .separator("→")
        .gap(12.)
        .items([
            sapid::breadcrumb_item("home", "Home").href("#home"),
            sapid::breadcrumb_item("guides", "Guides").href("#guides"),
            sapid::breadcrumb_item("article", "Getting started"),
        ])
}
