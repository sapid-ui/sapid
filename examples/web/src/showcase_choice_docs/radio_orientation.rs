use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::radio_group("radio-vertical")
            .theme(theme.clone())
            .label("Vertical delivery")
            .default_value("standard")
            .items([
                sapid::radio("standard").label("Standard"),
                sapid::radio("express").label("Express"),
            ]),
        sapid::radio_group("radio-horizontal")
            .theme(theme.clone())
            .label("Horizontal delivery")
            .orientation(glory::render::Orientation::Horizontal)
            .default_value("standard")
            .items([
                sapid::radio("standard").label("Standard"),
                sapid::radio("express").label("Express"),
            ]),
    ))
}
