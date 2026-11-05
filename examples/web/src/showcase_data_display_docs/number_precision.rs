use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::data_list()
        .theme(theme.clone())
        .label_width(100.)
        .item(
            sapid::data_list_item().term("Grouped").fill(
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(12345.678)
                    .max_fraction_digits(2),
            ),
        )
        .item(
            sapid::data_list_item().term("No grouping").fill(
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(12345.678)
                    .grouping(false)
                    .min_fraction_digits(3)
                    .max_fraction_digits(3),
            ),
        )
        .item(
            sapid::data_list_item().term("Percent").fill(
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(0.8542)
                    .format(sapid::NumberFormat::Percent)
                    .max_fraction_digits(1),
            ),
        )
}
