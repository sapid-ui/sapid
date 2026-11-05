use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let start = CivilDate::parse_iso("2024-02-18").unwrap();
    let range = HeatmapRange::new(start, start.checked_add_days(27).unwrap()).unwrap();
    let data = HeatmapData::new(
        (0..28)
            .filter(|i| i % 4 != 0)
            .map(|i| (range.date(i).unwrap(), (i % 9) as f64)),
    )
    .unwrap();
    heatmap("custom-calendar")
        .theme(theme.clone())
        .name("Localized contribution calendar")
        .range(range)
        .data(data)
        .locale(DateLocale::ZhCn)
        .week_start(Weekday::Monday)
        .weekday_labels(["日", "一", "二", "三", "四", "五", "六"].map(str::to_owned))
        .with_weekday_labels(true)
        .with_month_labels(true)
        .with_legend(true)
        .cell_size(20.)
        .gap(4.)
        .radius(4.)
        .empty_color(theme.surface)
        .colors([
            theme.accent_soft,
            theme.shades[4],
            theme.shades[7],
            theme.accent_text,
        ])
        .cell_style(move |cell| {
            if matches!(cell.date.weekday(), Weekday::Saturday | Weekday::Sunday) {
                Style {
                    border_color: Some(theme.danger),
                    ..Style::new()
                }
            } else {
                Style::new()
            }
        })
}
