use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill([sapid::AlertVariant::Light,sapid::AlertVariant::Filled,sapid::AlertVariant::Outline,sapid::AlertVariant::Transparent].into_iter().enumerate().map(|(i,variant)|sapid::alert(format!("variant-{i}")).theme(theme.clone()).variant(variant).announcement(sapid::Announcement::Off).title(format!("{variant:?} message")).content("The same message uses a different presentation while retaining its title and description relationships.").closable(true)).collect::<Vec<_>>())
}
