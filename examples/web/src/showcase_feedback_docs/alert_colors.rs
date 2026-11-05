use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
  sapid::stack().theme(theme.clone()).gap(12.).fill([sapid::AlertSeverity::Info,sapid::AlertSeverity::Success,sapid::AlertSeverity::Warning,sapid::AlertSeverity::Danger].into_iter().enumerate().map(|(i,severity)|sapid::alert(format!("severity-{i}")).theme(theme.clone()).severity(severity).announcement(sapid::Announcement::Off).title(format!("{severity:?} message")).content("Severity selects a finite theme palette; announcement policy is configured separately.")).collect::<Vec<_>>()),
  sapid::alert("custom-bright").theme(theme).variant(sapid::AlertVariant::Filled).color(Color(255,225,65,255)).announcement(sapid::Announcement::Off).title("A bright custom color").content("The filled title, message and close glyph choose a readable foreground.").closable(true),
 ))
}
