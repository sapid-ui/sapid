use glory::style::{Color, Length, Style};
use glory::{
    apace::{Config, Raster, Renderer},
    prelude::*,
    render::{Event, MotionPreference},
};
use sapid::{ContentPolicy, TransitionEffect, TransitionTiming, transition};
#[test]
fn custom_opacity_uses_the_same_native_current_phase() {
    let mut renderer = None;
    let app = App::new(|runtime| {
        let value = Renderer::with_config(
            runtime,
            Config {
                motion: MotionPreference::NoPreference,
                ..Default::default()
            },
        );
        renderer = Some(value.clone());
        value
    });
    let renderer = renderer.unwrap();
    renderer.set_viewport(160, 160, 1.).unwrap();
    let mounted = app.scope().cage(true);
    app.mount(
        transition()
            .mounted(mounted)
            .duration(200)
            .timing(TransitionTiming::Linear)
            .effect(TransitionEffect::Custom(sapid::TransitionStyles {
                opacity: 500,
                ..Default::default()
            }))
            .content(|| {
                glory::ui::view().style(Style {
                    height: Some(Length::Px(80.)),
                    width: Some(Length::Px(80.)),
                    background: Some(Color(255, 0, 0, 255)),
                    ..Style::new()
                })
            }),
    )
    .unwrap();
    app.settle(16).unwrap();
    pixels(&renderer);
    mounted.revise(|v| *v = false);
    app.flush().unwrap();
    renderer.advance_time(0.1).unwrap();
    let sample = red_pixel(&pixels(&renderer));
    assert!((180..=200).contains(&sample[3]), "{sample:?}");
    renderer.advance_time(0.2).unwrap();
    pixels(&renderer);
    app.pump().unwrap();
    assert_eq!(red_pixel(&pixels(&renderer))[3], 0);
}
use std::{cell::RefCell, rc::Rc};
#[test]
fn authored_css_hiding_settles_exit_and_restoring_the_ancestor_does_not_replay() {
    let mut renderer = None;
    let app = App::new(|runtime| {
        let value = Renderer::with_config(
            runtime,
            Config {
                motion: MotionPreference::NoPreference,
                ..Default::default()
            },
        );
        renderer = Some(value.clone());
        value
    });
    let renderer = renderer.unwrap();
    renderer.set_viewport(160, 160, 1.).unwrap();
    let mounted = app.scope().cage(true);
    let ancestor = app.scope().cage(Style::new());
    let editor = app.scope().cage(None);
    let completed = Rc::new(RefCell::new(0));
    let out = completed.clone();
    app.mount(
        glory::ui::view().style(ancestor).fill(
            transition()
                .mounted(mounted)
                .duration(200)
                .on_exited(move || *out.borrow_mut() += 1)
                .content(move || glory::ui::input().node_ref(editor)),
        ),
    )
    .unwrap();
    app.settle(16).unwrap();
    pixels(&renderer);
    mounted.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(editor.get().is_some());
    ancestor.revise(|style| style.display = Some(glory::style::Display::None));
    app.flush().unwrap();
    pixels(&renderer);
    app.pump().unwrap();
    assert!(editor.get().is_none());
    assert_eq!(*completed.borrow(), 1);
    ancestor.revise(|style| style.display = None);
    app.flush().unwrap();
    renderer.advance_time(1.).unwrap();
    pixels(&renderer);
    app.pump().unwrap();
    assert_eq!(*completed.borrow(), 1);
}
fn pixels(renderer: &Renderer) -> Vec<u8> {
    let mut raster = Raster::new();
    raster.draw(renderer).unwrap();
    raster.pixels().to_vec()
}
fn red_pixel(pixels: &[u8]) -> [u8; 4] {
    pixels[(10 * 160 + 10) * 4..(10 * 160 + 10) * 4 + 4]
        .try_into()
        .unwrap()
}
#[test]
fn real_native_opacity_delay_reversal_identity_and_revisioned_completion() {
    let mut backend = None;
    let app = App::new(|runtime| {
        let renderer = Renderer::with_config(
            runtime,
            Config {
                motion: MotionPreference::NoPreference,
                ..Default::default()
            },
        );
        backend = Some(renderer.clone());
        renderer
    });
    let renderer = backend.unwrap();
    renderer.set_viewport(160, 160, 1.).unwrap();
    let open = app.scope().cage(true);
    let reference = app.scope().cage(None);
    let body = app.scope().cage(None);
    let events = Rc::new(RefCell::new(Vec::new()));
    let completed = events.clone();
    app.mount(
        transition()
            .mounted(open)
            .node_ref(reference)
            .duration(200)
            .exit_delay(100)
            .timing(TransitionTiming::Linear)
            .content_policy(ContentPolicy::KeepMounted)
            .on_exited(move || completed.borrow_mut().push("exited"))
            .content(move || {
                glory::ui::view().node_ref(body).style(Style {
                    width: Some(Length::Px(80.)),
                    height: Some(Length::Px(80.)),
                    background: Some(Color(255, 0, 0, 255)),
                    ..Style::new()
                })
            }),
    )
    .unwrap();
    app.settle(16).unwrap();
    let root = reference.get().unwrap();
    let child = body.get().unwrap();
    let physical = renderer.engine_id(child, glory::html::Part::Main).unwrap();
    let full = red_pixel(&pixels(&renderer));
    assert!(full[0] > 240 && full[1] < 10 && full[2] < 10);
    open.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(!app.tree().active(child));
    renderer.advance_time(0.05).unwrap();
    let delayed = red_pixel(&pixels(&renderer));
    assert_eq!(delayed, full);
    renderer.advance_time(0.2).unwrap();
    let half = red_pixel(&pixels(&renderer));
    assert!(half[3] > 100 && half[3] < 160, "{half:?}");
    open.revise(|v| *v = true);
    app.flush().unwrap();
    let reversed = red_pixel(&pixels(&renderer));
    assert!(reversed[3].abs_diff(half[3]) <= 2);
    renderer.advance_time(0.4).unwrap();
    pixels(&renderer);
    app.pump().unwrap();
    assert!(events.borrow().is_empty());
    assert_eq!(
        renderer.engine_id(child, glory::html::Part::Main).unwrap(),
        physical
    );
    assert!(
        !app.dispatch(root, Event::PresenceComplete { revision: 1 })
            .unwrap()
    );
    open.revise(|v| *v = false);
    app.flush().unwrap();
    renderer.advance_time(0.8).unwrap();
    pixels(&renderer);
    app.pump().unwrap();
    assert_eq!(*events.borrow(), ["exited"]);
    assert_eq!(body.get(), Some(child));
    assert_eq!(red_pixel(&pixels(&renderer))[3], 0);
    app.pump().unwrap();
    assert_eq!(events.borrow().len(), 1);
}
#[test]
fn native_transform_rejection_occurs_before_enter_callbacks_and_reduce_settles() {
    let host = glory::apace::shell::WindowApp::new();
    host.renderer().set_viewport(160, 160, 1.).unwrap();
    assert!(
        host.mount(
            transition()
                .mounted(true)
                .effect(TransitionEffect::Scale)
                .content(|| glory::ui::text().content("unsupported"))
        )
        .is_err()
    );
    let host = glory::apace::shell::WindowApp::new();
    host.renderer().set_viewport(160, 160, 1.).unwrap();
    let open = host.app().scope().cage(true);
    let child = host.app().scope().cage(None);
    host.mount(
        transition()
            .mounted(open)
            .content(move || glory::ui::input().node_ref(child)),
    )
    .unwrap();
    open.revise(|v| *v = false);
    host.app().flush().unwrap();
    assert!(child.get().is_some());
    host.renderer()
        .set_motion_preference(MotionPreference::Reduce);
    pixels(host.renderer());
    host.app().pump().unwrap();
    assert!(child.get().is_none());
}
