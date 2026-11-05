use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::collapsible("retained-delivery")
        .theme(theme.clone())
        .label("Edit retained delivery draft")
        .height_transition(220)
        .content_policy(sapid::ContentPolicy::KeepMounted)
        .content({
            let theme = theme.clone();
            move || {
                sapid::field("retained-delivery-draft")
                    .theme(theme.clone())
                    .label("Retained delivery draft")
                    .control(|field| {
                        sapid::text_field()
                            .field(field)
                            .default_value("Please call reception.")
                    })
            }
        })
}
