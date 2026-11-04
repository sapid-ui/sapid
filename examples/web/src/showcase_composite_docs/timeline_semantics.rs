use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::timeline("delivery-timeline")
        .theme(theme)
        .name("Order delivery")
        .active("transit")
        .heading_level(4)
        .state_labels("Done", "Now", "Next")
        .item(
            sapid::timeline_item("packed")
                .title("Order packed")
                .timestamp("09:15")
                .description("Your order is ready to leave the warehouse."),
        )
        .item(
            sapid::timeline_item("transit")
                .title("In transit")
                .timestamp("12:30")
                .description("The carrier has collected the parcel."),
        )
        .item(
            sapid::timeline_item("delivered").title("Delivered").fill(
                sapid::anchor()
                    .label("Track your delivery")
                    .href("#tracking"),
            ),
        )
}
