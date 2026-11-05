use super::showcase_examples::DetailSection;
mod appearance;
mod data;
mod domain;
mod range;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident,$id:literal,$title:literal,$note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    vec![
        section!(
            data,
            "heatmap-data",
            "Dated data, missing records and zero",
            "HeatmapData maps CivilDate to bounded finite values. Missing records remain distinct from an explicit zero in color, accessible names, tooltips and callbacks. set(date, Some(value)) updates a record; set(date, None) removes it. Sparse data may cover dates outside the visible range; automatic domains inspect only the current range. Data changes retain current date cells."
        ),
        section!(
            domain,
            "heatmap-domain",
            "Automatic and controlled value domains",
            "The default domain uses the minimum and maximum visible records. domain accepts a read-only Cage/Bond/Vary of Option<HeatmapDomain>; None restores automatic scaling. Colors clamp beyond the domain, and an equal-endpoint domain uses the first color. label_format supplies bounded tooltip and accessible labels; zero and missing values remain application-visible."
        ),
        section!(
            appearance,
            "heatmap-appearance",
            "Colors, geometry and localized labels",
            "Configure one to eight colors, empty_color, 8–32px cell_size, 0–8px gap and a bounded radius. Week columns and weekday rows come from civil arithmetic. week_start changes display order; weekday_labels always accepts Sunday–Saturday and month_labels January–December. English/Chinese defaults and per-cell typed style are independent of data. The legend shows the active numeric domain."
        ),
        section!(
            range,
            "heatmap-range",
            "Date ranges, keyboard and native boundaries",
            "Heatmap requires an explicit HeatmapRange of 1–366 days rather than sampling a clock. Range updates retain overlapping keyed dates and retire removed cells. A full year scrolls locally. One available cell is a Tab stop; arrows follow weeks/weekday rows, Home/End reach that weekday's edges and PageUp/PageDown move four weeks. Enter/Space invoke on_day_click with the current optional value. Browser tooltips are optional; Apace supports the dated cells, keyboard, labels and painting but explicitly rejects with_tooltip(true). No arbitrary DOM rectangle props or OS reader acceptance is implied."
        ),
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Data and zero values", "#heatmap-data"),
        ("Value domains", "#heatmap-domain"),
        ("Colors and labels", "#heatmap-appearance"),
        ("Range and keyboard", "#heatmap-range"),
    ]
}
