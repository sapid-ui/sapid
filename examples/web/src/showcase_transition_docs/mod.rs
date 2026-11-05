use super::showcase_examples::DetailSection;
mod custom;
mod effects;
mod lifecycle;
mod policies;
mod reduced;
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
            custom,
            "transition-custom",
            "Custom closed styles and transform origin",
            "TransitionEffect::Custom accepts a validated TransitionStyles declaration for the closed state; open is full opacity with an identity transform. Opacity and independent X/Y scales use thousandths (0–1000 and 0–2000), translation is bounded to ±200px, rotation to ±180°, skew to ±45°, and origin coordinates to 0–100 percent. Put common background, border and layout on the content. The browser interpolates these properties together; native supports custom opacity without transforms. No arbitrary unvalidated CSS string enters the semantic protocol."
        ),
        section!(
            effects,
            "transition-effects",
            "Effects and timing",
            "mounted is a read-only Cage/Bond/Vary source. Fade (including directional fades), Scale/ScaleX/ScaleY, four Slide directions, SkewUp/Down, RotateLeft/Right and Pop with corner origins share a bounded presence shell. None settles immediately. duration and exit_duration accept 0–2000ms, distance accepts 0–200px, and timing selects a finite CSS easing curve. Changing accepted configuration retargets the current presentation; it does not recreate the content."
        ),
        section!(
            lifecycle,
            "transition-lifecycle",
            "Delays, completion and interruption",
            "enter_delay and exit_delay are independent 0–2000ms values. on_enter/on_exit run when a new configuration is accepted; on_entered/on_exited run once after its matching host completion. A reversal invalidates the old completion and continues from current pixels. Closed content is immediately hidden from input and accessibility even while exit pixels remain. Retiring the Owner cancels pending completion."
        ),
        section!(
            policies,
            "transition-content",
            "Keep mounted and unmount policies",
            "Unmount is the default: retain content during exit, then dispose its Owner after completion. Reopening builds fresh local state. KeepMounted preserves the same editor and value through hidden intervals. The shell remains present. Put padding, borders, backgrounds and layout on content; Transition owns its shell opacity, transform and display."
        ),
        section!(
            reduced,
            "transition-motion",
            "Reduced motion, SSR and native support",
            "animated(false), zero duration and the host reduced-motion preference settle without animation. Initial open content settles by default; appear(true) opts into first-entry animation. SSR and headless renderers produce the accepted static state and complete lifecycle synchronously. The browser host supports all finite effects; Apace supports actual fade/none and custom opacity, and explicitly rejects animated transforms. Wry rejects presence. This is not a claim of OS input or screen-reader acceptance."
        ),
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Custom closed styles and transform origin",
            "#transition-custom",
        ),
        ("Effects and timing", "#transition-effects"),
        (
            "Delays, completion and interruption",
            "#transition-lifecycle",
        ),
        ("Keep mounted and unmount policies", "#transition-content"),
        (
            "Reduced motion, SSR and native support",
            "#transition-motion",
        ),
    ]
}
