use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::segmented_control("billing-interval")
        .theme(theme)
        .label("Billing interval")
        .default_value("monthly")
        .segments([
            sapid::segment("monthly").label("Monthly"),
            sapid::segment("yearly").label("Yearly"),
            sapid::segment("lifetime").label("Lifetime"),
        ])
}
