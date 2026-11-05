use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stepper("delivery-workflow")
        .theme(theme)
        .name("Delivery progress")
        .orientation(glory::render::Orientation::Vertical)
        .default_active(1)
        .selectable(true)
        .steps([
            sapid::step("received")
                .label("Received")
                .description("Your request was received"),
            sapid::step("processing")
                .label("Processing")
                .description("We are preparing the result"),
            sapid::step("delivered")
                .label("Delivered")
                .description("Available after confirmation")
                .enabled(false),
        ])
}
