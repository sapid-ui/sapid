use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P6-07", theme)
}

#[allow(dead_code, unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::FlexWrap;
    use sapid::Theme;

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use glory::render::{Orientation, ScrollAxes, ScrollCommand, ScrollMovement};
    use glory::style::{Color, ObjectFit};
    use sapid::{OverscrollBehavior, ScrollbarOffset, Scrollbars, Theme};
    use std::{cell::RefCell, rc::Rc, time::Duration};

    pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
        match id {
            "P3-04" => Box::new(aspect_demo(theme)),
            "P6-07" => Box::new(scroll_demo(theme)),
            "P6-08" => Box::new(splitter_demo(theme)),
            _ => unreachable!("layout demo"),
        }
    }

    fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> impl Widget {
        view()
            .class("site-layout-playground")
            .fill(super::controls::playground(preview, controls))
    }

    fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label.to_owned()),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label.to_owned())
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .value_text(value.map(|value| Some(format!("{value:.0}"))))
                    .on_value_change(move |next| value.revise(|value| *value = next)),
            )),
        )
    }

    fn heading(value: &str, theme: &Theme) -> impl Widget {
        sapid::title(3)
            .theme(theme.clone())
            .class("site-demo-heading")
            .content(value.to_owned())
    }

    fn aspect_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let ratio = scope.cage(Some("3-2".to_owned()));
            let width = scope.cage(300.0);
            let value = ratio.map(|ratio| match ratio.as_deref() {
                Some("16-9") => 16.0 / 9.0,
                Some("4-3") => 4.0 / 3.0,
                Some("1-1") => 1.0,
                Some("9-16") => 9.0 / 16.0,
                _ => 1.5,
            });
            let preview = sapid::aspect_ratio(value)
                .theme(theme.clone())
                .class("site-aspect-preview")
                .style(width.map(|width| {
                    Style::new()
                        .width(Length::Px(*width))
                        .max_width(Limit::Percent(100.0))
                }))
                .fill(
                    sapid::image(format!(
                        "{}/image-pattern.png",
                        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                    ))
                    .theme(theme.clone())
                    .alt("Abstract color study")
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .fit(ObjectFit::Cover),
                );
            sapid::stack().theme(theme.clone()).gap(24.0).fill((
            playground(Box::new(preview), vec![
                super::controls::select("aspect-ratio", "Ratio", ratio,
                    &[("16-9", "16 / 9"), ("3-2", "3 / 2"), ("4-3", "4 / 3"), ("1-1", "1 / 1"), ("9-16", "9 / 16")], &theme),
                range("aspect-width", "Width", width, 100.0, 600.0, 10.0, &theme),
            ]),
            sapid::text().theme(theme.clone()).content("The image fills its frame while preserving the selected ratio. Resize the browser to see the frame shrink with its container."),
            heading("Inside a flex container", &theme),
            sapid::group().theme(theme.clone()).gap(16.0).fill((
                sapid::aspect_ratio(1.0).theme(theme.clone()).width(Length::Px(100.0))
                    .style(Style { background: Some(theme.accent_soft), ..Style::new().radius(theme.radius) })
                    .fill(sapid::text().theme(theme.clone()).content("1 : 1")),
                sapid::text().theme(theme.clone()).content("Set a width when placing a ratio frame in a flex layout. Children are centered inside the reserved frame and cannot increase its height."),
            )),
        )).show_in(scope)
        })
    }

    fn pane(label: impl Into<Vary<String>>, color: Color) -> impl Widget {
        view()
            .class("site-splitter-pane")
            .style(Style {
                background: Some(color),
                color: Some(Color(255, 255, 255, 255)),
                align: Some(Align::Center),
                justify: Some(Justify::Center),
                ..Style::new()
                    .display(Display::Flex)
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .min_width(Length::Px(0.0))
                    .min_height(Length::Px(0.0))
            })
            .fill(text().content(label))
    }

    fn splitter_pair<A: Into<Vary<String>>, B: Into<Vary<String>>>(
        theme: &Theme,
        key: &'static str,
        name: &'static str,
        first: A,
        second: B,
    ) -> sapid::Splitter {
        let first = first.into();
        let second = second.into();
        sapid::splitter(key)
            .theme(theme.clone())
            .name(name)
            .first({
                let first = first.clone();
                move || pane(first.clone(), Color(34, 139, 230, 255))
            })
            .second(move || pane(second.clone(), Color(18, 184, 134, 255)))
    }

    fn splitter_extra_demos(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let fixed_sidebar = view()
                .class("site-splitter-fixed-sidebar")
                .style(Style {
                    shrink: Some(0.0),
                    ..Style::new()
                        .width(Length::Px(240.0))
                        .max_width(Limit::Percent(45.0))
                })
                .fill(pane("Fixed 240px sidebar", Color(34, 139, 230, 255)));
            let fixed = view()
                .class("site-splitter-fixed-layout")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .width(Length::Percent(100.0))
                        .height(Length::Px(220.0)),
                )
                .fill((
                    fixed_sidebar,
                    splitter_pair(
                        &theme,
                        "showcase-fixed-content",
                        "Resize flexible content",
                        "Editor",
                        "Terminal",
                    )
                    .axis(Orientation::Vertical)
                    .default_value(60.0)
                    .min(20.0)
                    .max(80.0)
                    .style(Style {
                        grow: Some(1.0),
                        min_width: Some(Length::Px(0.0)),
                        ..Style::new().height(Length::Percent(100.0))
                    }),
                ));
            let multiple = splitter_pair(
                &theme,
                "showcase-multiple",
                "Resize workspace groups",
                "Panel A",
                "Panel B",
            )
            .class("site-splitter-multiple")
            .default_value(45.0)
            .min(20.0)
            .max(80.0)
            .style(Style::new().height(Length::Px(260.0)))
            .first({
                let theme = theme.clone();
                move || {
                    splitter_pair(
                        &theme,
                        "showcase-multiple-left",
                        "Resize left workspaces",
                        "Panel A",
                        "Panel B",
                    )
                    .axis(Orientation::Vertical)
                    .default_value(50.0)
                    .min(20.0)
                    .max(80.0)
                    .style(Style::new().height(Length::Percent(100.0)))
                }
            })
            .second({
                let theme = theme.clone();
                move || {
                    splitter_pair(
                        &theme,
                        "showcase-multiple-right",
                        "Resize right workspaces",
                        "Panel C",
                        "Panel D",
                    )
                    .axis(Orientation::Vertical)
                    .default_value(55.0)
                    .min(20.0)
                    .max(80.0)
                    .style(Style::new().height(Length::Percent(100.0)))
                }
            });
            let outer = scope.cage(30.0);
            let inner = scope.cage(65.0);
            let redistribute = splitter_pair(
                &theme,
                "showcase-redistribute",
                "Resize redistributed panels",
                "Panel A",
                "Panel B",
            )
            .class("site-splitter-redistribute")
            .value(outer)
            .min(15.0)
            .max(70.0)
            .style(Style::new().height(Length::Px(240.0)))
            .on_value_change(move |next| outer.revise(|value| *value = next))
            .second({
                let theme = theme.clone();
                move || {
                    splitter_pair(
                        &theme,
                        "showcase-redistribute-right",
                        "Resize nested panels",
                        "Panel B",
                        "Panel C",
                    )
                    .axis(Orientation::Vertical)
                    .value(inner)
                    .min(20.0)
                    .max(80.0)
                    .style(Style::new().height(Length::Percent(100.0)))
                    .on_value_change(move |next| inner.revise(|value| *value = next))
                }
            });
            let sizes = scope.bond(move || {
                let a = outer.get();
                let b = inner.get() * (100.0 - a) / 100.0;
                let c = 100.0 - a - b;
                format!("Current sizes: [{a:.0}, {b:.0}, {c:.0}]")
            });
            let presets = sapid::group().theme(theme.clone()).gap(8.0).fill(
                [
                    ("Balanced", 50.0, 50.0),
                    ("Editor focus", 30.0, 70.0),
                    ("Terminal focus", 65.0, 35.0),
                ]
                .into_iter()
                .map(|(label, a, b)| {
                    sapid::button()
                        .theme(theme.clone())
                        .size(sapid::Size::Small)
                        .label(label)
                        .on_press(move || {
                            outer.revise(|value| *value = a);
                            inner.revise(|value| *value = b);
                        })
                })
                .collect::<Vec<_>>(),
            );
            sapid::stack()
            .theme(theme.clone())
            .gap(24.0)
            .fill((
                heading("Fixed sidebar and flexible content", &theme),
                sapid::text().theme(theme.clone()).content("Resizable panes use percentages. A fixed-width sidebar can be composed beside a flexible Splitter."),
                fixed,
                heading("Multiple panes", &theme),
                sapid::text().theme(theme.clone()).content("Compose nested splitters to create any number of workspaces."),
                multiple,
                heading("Redistribute with controlled sizes", &theme),
                sapid::text().theme(theme.clone()).content("Coordinated controlled values let the application redistribute space across all regions."),
                redistribute,
                presets,
                sapid::text().theme(theme.clone()).class("site-splitter-redistribute-sizes").content(sizes),
            ))
            .show_in(scope)
        })
    }

    fn splitter_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let orientation = scope.cage(Some("horizontal".to_owned()));
            let value = scope.cage(50.0);
            let disabled = scope.cage(false);
            let collapsible = scope.cage(false);
            let restored = scope.cage(50.0);
            let line_size = scope.cage(6.0);
            let with_grip = scope.cage(true);
            let axis = orientation.map(|value| {
                if value.as_deref() == Some("vertical") {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                }
            });
            let preview = sapid::splitter("showcase-splitter")
                .theme(theme.clone())
                .name("Resize panes")
                .class("site-splitter-preview")
                .axis(axis)
                .line_size(line_size)
                .with_grip(with_grip)
                .value(value)
                .enabled(disabled.map(|disabled| !disabled))
                .min(collapsible.map(|enabled| if *enabled { 0.0 } else { 20.0 }))
                .max(80.0)
                .style(scope.bond(move || {
                    Style::new()
                        .width(Length::Percent(100.0))
                        .height(Length::Px(if axis.get() == Orientation::Vertical {
                            300.0
                        } else {
                            200.0
                        }))
                }))
                .first(move || {
                    pane(
                        value.map(|value| format!("First pane ({value:.0}%)")),
                        Color(34, 139, 230, 255),
                    )
                })
                .second(move || {
                    pane(
                        value.map(|value| format!("Second pane ({:.0}%)", 100.0 - value)),
                        Color(18, 184, 134, 255),
                    )
                })
                .on_value_change(move |next| value.revise(|value| *value = next));
            let controls: Vec<Box<dyn Widget>> = vec![
                super::controls::select(
                    "splitter-orientation",
                    "Orientation",
                    orientation,
                    &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                    &theme,
                ),
                range(
                    "splitter-line-size",
                    "Line thickness",
                    line_size,
                    0.0,
                    6.0,
                    1.0,
                    &theme,
                ),
                super::controls::switch("Show grip", with_grip, &theme),
                super::controls::switch("Disabled", disabled, &theme),
                Box::new(
                    sapid::switch()
                        .theme(theme.clone())
                        .label("Collapsible")
                        .checked(collapsible)
                        .on_checked_change(move |next| {
                            if !next && value.get_untracked() < 20.0 {
                                value.revise(|value| *value = 50.0);
                            }
                            collapsible.revise(|value| *value = next);
                        }),
                ),
                Box::new(
                    sapid::text()
                        .theme(theme.clone())
                        .class("site-splitter-sizes")
                        .content(value.map(|value| {
                            format!("Current sizes: [{value:.0}, {:.0}]", 100.0 - value)
                        })),
                ),
                Box::new(
                    sapid::group().theme(theme.clone()).gap(6.0).fill(
                        [30.0, 50.0, 70.0]
                            .into_iter()
                            .map(|next| {
                                sapid::button()
                                    .theme(theme.clone())
                                    .size(sapid::Size::ExtraSmall)
                                    .label(format!("{next:.0} / {:.0}", 100.0 - next))
                                    .on_press(move || value.revise(|value| *value = next))
                            })
                            .collect::<Vec<_>>(),
                    ),
                ),
                Box::new(
                    sapid::button()
                        .theme(theme.clone())
                        .enabled(collapsible)
                        .label(value.map(|value| {
                            if *value == 0.0 {
                                "Expand first pane".to_owned()
                            } else {
                                "Collapse first pane".to_owned()
                            }
                        }))
                        .on_press(move || {
                            let old = value.get_untracked();
                            if old == 0.0 {
                                value.revise(|value| *value = restored.get_untracked());
                            } else {
                                restored.revise(|value| *value = old);
                                value.revise(|value| *value = 0.0);
                            }
                        }),
                ),
            ];
            sapid::stack().theme(theme.clone()).gap(24.0).fill((
            playground(Box::new(preview), controls),
            sapid::text().theme(theme.clone()).content("Drag the divider, or focus it and use the arrow keys, Home and End. Orientation and controlled sizes update the existing panes."),
            heading("Nested splitters", &theme),
            sapid::splitter("showcase-nested-outer").theme(theme.clone()).name("Resize sidebar").default_value(30.0).min(15.0).max(70.0)
                .style(Style::new().height(Length::Px(300.0)))
                .first(|| pane("Sidebar", Color(34,139,230,255)))
                .second({ let theme = theme.clone(); move || sapid::splitter("showcase-nested-inner").theme(theme.clone()).name("Resize editor and terminal").axis(Orientation::Vertical).default_value(65.0).min(20.0).max(80.0)
                    .style(Style::new().height(Length::Percent(100.0)))
                    .first(|| pane("Editor", Color(18,184,134,255)))
                    .second(|| pane("Terminal", Color(190,75,219,255))) }),
            splitter_extra_demos(theme.clone()),
        )).show_in(scope)
        })
    }

    fn scroll_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let behavior = scope.cage(Some("hover".to_owned()));
            let offsets = scope.cage(Some("none".to_owned()));
            let axes = scope.cage(Some("xy".to_owned()));
            let overscroll = scope.cage(Some("auto".to_owned()));
            let size = scope.cage(12.0);
            let delay = scope.cage(500.0);
            let wide = scope.cage(false);
            let draft = scope.cage("Keep this draft while scrolling".to_owned());
            let position = scope.cage((0.0, 0.0));
            let handle: Rc<RefCell<Option<sapid::ScrollHandle>>> = Rc::new(RefCell::new(None));
            let saved = handle.clone();
            let content_theme = theme.clone();
            let preview = sapid::scroll_area("showcase-scroll-area").theme(theme.clone()).name("Article viewport").class("site-scroll-preview")
            .axes(axes.map(|value| match value.as_deref() { Some("x") => ScrollAxes::Horizontal, Some("y") => ScrollAxes::Vertical, _ => ScrollAxes::Both }))
            .scrollbars(behavior.map(|value| match value.as_deref() { Some("always") => Scrollbars::Always, Some("auto") => Scrollbars::Auto, Some("scroll") => Scrollbars::Scroll, Some("never") => Scrollbars::Never, _ => Scrollbars::Hover }))
            .scrollbar_size(size)
            .offset_scrollbars(offsets.map(|value| match value.as_deref() { Some("both") => ScrollbarOffset::Both, Some("x") => ScrollbarOffset::Horizontal, Some("y") => ScrollbarOffset::Vertical, Some("present") => ScrollbarOffset::Present, _ => ScrollbarOffset::None }))
            .overscroll_behavior(overscroll.map(|value| match value.as_deref() { Some("contain") => OverscrollBehavior::Contain, Some("none") => OverscrollBehavior::None, _ => OverscrollBehavior::Auto }))
            .scroll_hide_delay(delay.map(|value| Duration::from_millis(*value as u64)))
            .delay(scroll_delay)
            .style(Style::new().width(Length::Percent(100.0)).height(Length::Px(250.0)))
            .on_ready(move |value| *saved.borrow_mut() = Some(value))
            .on_scroll_change(move |next| position.revise(|value| *value = next))
            .content(move |_| sapid::stack().theme(content_theme.clone()).gap(16.0)
                .style(wide.map(|wide| Style::new().width(if *wide { Length::Px(750.0) } else { Length::Percent(100.0) })))
                .fill((
                    sapid::title(4).theme(content_theme.clone()).content("Designing a comfortable reading space"),
                    sapid::text().theme(content_theme.clone()).content("A scrolling region gives long content its own space while the rest of the interface stays in view. Use the controls to explore the scrollbar behavior and available axes."),
                    sapid::text_field().theme(content_theme.clone()).name("Scroll area draft").value(draft).on_value_change(move |next| draft.revise(|value| *value = next)),
                    glory::ui::group().fill((1..=7).map(|index| sapid::text().theme(content_theme.clone()).content(format!("Section {index}. Give the reader a clear heading, comfortable spacing and a predictable way to move through the document. The native viewport keeps wheel, touch, selection and keyboard behavior. Changing the scrollbar presentation keeps this content and its editing state mounted."))).collect::<Vec<_>>()),
                )));
            let controls = vec![
                super::controls::select(
                    "scroll-type",
                    "Type",
                    behavior,
                    &[
                        ("hover", "Hover"),
                        ("scroll", "Scroll"),
                        ("auto", "Auto"),
                        ("always", "Always"),
                        ("never", "Never"),
                    ],
                    &theme,
                ),
                super::controls::select(
                    "scroll-offset",
                    "Offset scrollbars",
                    offsets,
                    &[
                        ("none", "None"),
                        ("both", "Both"),
                        ("x", "Horizontal"),
                        ("y", "Vertical"),
                        ("present", "Present"),
                    ],
                    &theme,
                ),
                super::controls::select(
                    "scroll-axes",
                    "Scrollbars",
                    axes,
                    &[
                        ("xy", "Both axes"),
                        ("y", "Vertical only"),
                        ("x", "Horizontal only"),
                    ],
                    &theme,
                ),
                super::controls::select(
                    "scroll-overscroll",
                    "Overscroll behavior",
                    overscroll,
                    &[("auto", "Auto"), ("contain", "Contain"), ("none", "None")],
                    &theme,
                ),
                range(
                    "scroll-size",
                    "Scrollbar size",
                    size,
                    2.0,
                    24.0,
                    1.0,
                    &theme,
                ),
                range(
                    "scroll-delay",
                    "Scroll hide delay",
                    delay,
                    0.0,
                    2000.0,
                    100.0,
                    &theme,
                ),
                super::controls::switch("Wide content", wide, &theme),
            ];
            let mut actions: Vec<Box<dyn Widget>> = Vec::new();
            for (label, movement) in [
                ("Scroll to top", Some(ScrollMovement::Start)),
                ("Scroll to center", None),
                ("Scroll to bottom", Some(ScrollMovement::End)),
            ] {
                let handle = handle.clone();
                let weak = scope.downgrade();
                actions.push(Box::new(
                    sapid::button()
                        .theme(theme.clone())
                        .size(sapid::Size::Small)
                        .label(label)
                        .enabled(axes.map(|value| value.as_deref() != Some("x")))
                        .on_press(move || {
                            let handle = handle.borrow().clone();
                            if let (Some(handle), Some(scope)) = (handle, weak.upgrade()) {
                                let result = if let Some(movement) = movement {
                                    handle.command(ScrollCommand {
                                        axis: Orientation::Vertical,
                                        movement,
                                    })
                                } else {
                                    handle.scroll_to_fraction(0.0, 0.5)
                                };
                                if let Err(error) = result {
                                    scope.report_error(error);
                                }
                            }
                        }),
                ));
            }
            sapid::stack()
                .theme(theme.clone())
                .gap(18.0)
                .fill((
                    playground(Box::new(preview), controls),
                    sapid::text()
                        .theme(theme.clone())
                        .class("site-scroll-position")
                        .content(
                            position.map(|(x, y)| format!("Scroll position: x {x:.0}, y {y:.0}")),
                        ),
                    sapid::group().theme(theme.clone()).gap(8.0).fill(actions),
                ))
                .show_in(scope)
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn scroll_delay(_duration: Duration) {}
    #[cfg(target_arch = "wasm32")]
    fn scroll_delay(duration: Duration) -> timer::Delay {
        timer::Delay::new(duration.as_millis() as i32)
    }

    #[cfg(target_arch = "wasm32")]
    mod timer {
        use std::{
            cell::{Cell, RefCell},
            future::Future,
            pin::Pin,
            rc::Rc,
            task::{Context, Poll, Waker},
        };
        use wasm_bindgen::{JsCast, prelude::*};
        struct State {
            done: Cell<bool>,
            waker: RefCell<Option<Waker>>,
        }
        pub(super) struct Delay {
            millis: i32,
            state: Rc<State>,
            timer: Option<i32>,
            callback: Option<Closure<dyn FnMut()>>,
        }
        impl Delay {
            pub fn new(millis: i32) -> Self {
                Self {
                    millis,
                    state: Rc::new(State {
                        done: Cell::new(false),
                        waker: RefCell::new(None),
                    }),
                    timer: None,
                    callback: None,
                }
            }
        }
        impl Future for Delay {
            type Output = ();
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                if self.state.done.get() {
                    return Poll::Ready(());
                }
                let previous = self.state.waker.borrow_mut().replace(cx.waker().clone());
                drop(previous);
                if self.timer.is_none() {
                    let state = self.state.clone();
                    let callback = Closure::new(move || {
                        state.done.set(true);
                        let waker = state.waker.borrow_mut().take();
                        if let Some(waker) = waker {
                            waker.wake();
                        }
                    });
                    self.timer = Some(
                        web_sys::window()
                            .expect("browser timer host")
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                callback.as_ref().unchecked_ref(),
                                self.millis,
                            )
                            .expect("browser timeout"),
                    );
                    self.callback = Some(callback);
                }
                Poll::Pending
            }
        }
        impl Drop for Delay {
            fn drop(&mut self) {
                if let Some(timer) = self.timer.take()
                    && let Some(window) = web_sys::window()
                {
                    window.clear_timeout_with_handle(timer);
                }
            }
        }
    }
}
