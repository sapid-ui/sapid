#![cfg(feature = "files")]
use glory::{
    apace::{
        Config, Raster,
        files::{FilePicker, FileReader, Files, Reply, SelectedFile},
        shell::WindowApp,
    },
    render::*,
};
use std::{
    sync::{Arc, Mutex},
    task::{Context, Waker},
    time::{Duration, Instant},
};
#[path = "../../web/src/dedicated_demos/file_input.rs"]
mod file_primary;
struct Reader(Arc<Vec<u8>>);
impl FileReader for Reader {
    fn read(&self, offset: u64, length: u32) -> Reply<Vec<u8>> {
        let bytes = self.0.clone();
        Box::pin(async move {
            bytes
                .get(offset as usize..offset as usize + length as usize)
                .map(<[u8]>::to_vec)
                .ok_or(FileRejection::Bytes)
        })
    }
}
#[derive(Default)]
struct Picker(Mutex<Option<Result<Option<Vec<SelectedFile>>, FileRejection>>>);
impl FilePicker for Picker {
    fn pick(&self, _: FileConfig) -> Reply<Option<Vec<SelectedFile>>> {
        Box::pin(std::future::ready(self.0.lock().unwrap().take().unwrap()))
    }
}
fn themes() -> [sapid::Theme; 4] {
    [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ]
}
fn find(host: &WindowApp, class: &str) -> Vec<NodeId> {
    let tree = host.app().tree();
    let mut pending = tree.roots().to_vec();
    let mut found = Vec::new();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|c| c == class) {
            found.push(id);
        }
        pending.extend(&node.children);
    }
    found
}
fn settle(host: &WindowApp) {
    host.observe(&Context::from_waker(Waker::noop())).unwrap();
}
fn pixels(host: &WindowApp) -> Vec<u8> {
    let mut raster = Raster::new();
    raster.draw(host.renderer()).unwrap();
    assert_eq!(raster.pixels().len(), 640 * 480 * 4);
    raster.pixels().to_vec()
}
fn png() -> Vec<u8> {
    include_bytes!("../../web/assets/image-pattern.png").to_vec()
}
fn pick(host: &WindowApp) {
    let node = find(host, "sapid-file-button")[0];
    host.app().dispatch(node, Event::Activate).unwrap();
    settle(host);
}
#[test]
fn native_input_summary_long_names_cancel_and_inset_clear_keep_the_same_picker() {
    for theme in themes() {
        let picker = Arc::new(Picker::default());
        let files = Files::new(picker.clone());
        let host = WindowApp::with_config(Config {
            files: Some(files.clone()),
            ..Default::default()
        });
        host.renderer().set_viewport(640, 480, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(glory::style::Style::new().width(glory::style::Length::Px(320.)))
                .fill(file_primary::demo(theme.clone())),
        )
        .unwrap();
        settle(&host);
        let id = find(&host, "sapid-file-button")[0];
        let engine = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        let name = format!("{}-project-notes.txt", "descriptive-".repeat(30));
        *picker.0.lock().unwrap() = Some(Ok(Some(vec![
            SelectedFile {
                name: name.clone(),
                media_type: "text/plain".into(),
                size: 1,
                reader: Arc::new(Reader(Arc::new(vec![1]))),
            },
            SelectedFile {
                name: "second.txt".into(),
                media_type: "text/plain".into(),
                size: 1,
                reader: Arc::new(Reader(Arc::new(vec![2]))),
            },
        ])));
        pick(&host);
        assert_eq!(files.stats().references, 2);
        assert!(
            matches!(host.app().tree().node(id).unwrap().descriptor.view().props,Props::File(p)if p.label==format!("{name}, +1 more"))
        );
        assert_eq!(
            host.app()
                .tree()
                .node(id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref(),
            Some("Attachments")
        );
        *picker.0.lock().unwrap() = Some(Ok(None));
        pick(&host);
        assert_eq!(files.stats().references, 2);
        let clear = find(&host, "sapid-file-input-clear")[0];
        let rect = |node| {
            let saved = std::rc::Rc::new(std::cell::RefCell::new(None));
            let out = saved.clone();
            host.app()
                .request(Query::Bounds(node), move |v| {
                    *out.borrow_mut() = Some(v.unwrap())
                })
                .unwrap();
            host.app().pump().unwrap();
            let QueryValue::Bounds(rect) = saved.borrow_mut().take().unwrap() else {
                panic!("bounds")
            };
            rect
        };
        let trigger = rect(id);
        let button = rect(clear);
        assert!(trigger.width <= 320. && button.width <= 25. && button.height <= 25.);
        assert!(button.x >= trigger.x && button.x + button.width <= trigger.x + trigger.width + 1.);
        assert!(
            button.y >= trigger.y && button.y + button.height <= trigger.y + trigger.height + 1.
        );
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        for y in (button.y + 4.) as usize..(button.y + 20.) as usize {
            for x in (button.x - 4.) as usize..(button.x - 1.) as usize {
                let offset = (y * 640 + x) * 4;
                assert_eq!(
                    &raster.pixels()[offset..offset + 4],
                    &[theme.input.0, theme.input.1, theme.input.2, theme.input.3],
                    "filename must not paint under the clear action"
                );
            }
        }
        host.app().scope().action(Action::Focus(clear)).unwrap();
        host.app().settle(16).unwrap();
        host.app().dispatch(clear, Event::Activate).unwrap();
        settle(&host);
        let focused = std::rc::Rc::new(std::cell::RefCell::new(None));
        let out = focused.clone();
        host.app()
            .request(Query::Focus(id), move |v| {
                *out.borrow_mut() = Some(v.unwrap())
            })
            .unwrap();
        host.app().pump().unwrap();
        assert_eq!(
            focused.borrow_mut().take(),
            Some(QueryValue::Focus(true)),
            "clearing returns focus to the same picker"
        );
        assert_eq!(files.stats().references, 0);
        assert!(
            matches!(host.app().tree().node(id).unwrap().descriptor.view().props,Props::File(p)if p.label=="Choose files")
        );
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            engine
        );
        host.app().clear().unwrap();
    }
}
#[test]
fn actual_file_input_reads_previews_removes_and_clears_across_four_themes() {
    for theme in themes() {
        let picker = Arc::new(Picker::default());
        let files = Files::new(picker.clone());
        let host = WindowApp::with_config(Config {
            files: Some(files.clone()),
            ..Default::default()
        });
        host.renderer().set_viewport(640, 480, 1.).unwrap();
        let scope = host.app().scope();
        let controller = scope.cage(None);
        let selected = scope.cage(Vec::<FileInfo>::new());
        let status = scope.cage(sapid::FileStatus::Idle);
        host.mount(
            sapid::file_input()
                .theme(theme)
                .label("Choose image")
                .configuration(FileConfig {
                    preview: Some(Default::default()),
                    ..Default::default()
                })
                .previews(true)
                .controller_ref(controller)
                .files_ref(selected)
                .status_ref(status),
        )
        .unwrap();
        let before = pixels(&host);
        let bytes = Arc::new(png());
        *picker.0.lock().unwrap() = Some(Ok(Some(vec![SelectedFile {
            name: "中文-😀.png".into(),
            media_type: "image/png".into(),
            size: bytes.len() as u64,
            reader: Arc::new(Reader(bytes.clone())),
        }])));
        let control = controller.get().unwrap();
        pick(&host);
        assert_eq!(status.get(), sapid::FileStatus::Selected);
        let file = selected.get()[0].token;
        let read = scope.cage(None);
        control
            .read(
                FileRead {
                    file,
                    offset: 0,
                    length: 8,
                },
                move |result| read.revise(|v| *v = Some(result)),
            )
            .unwrap();
        settle(&host);
        assert_eq!(read.get().unwrap().unwrap(), bytes[..8]);
        let preview = find(&host, "sapid-file-preview-request")[0];
        host.app().dispatch(preview, Event::Activate).unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            settle(&host);
            if host.renderer().resource_stats().cached_images == 1 {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "actual file attachment preview did not decode"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(find(&host, "sapid-file-preview").len(), 1);
        assert_ne!(before, pixels(&host));
        let remove = find(&host, "sapid-file-remove")[0];
        host.app().dispatch(remove, Event::Activate).unwrap();
        settle(&host);
        assert!(selected.get().is_empty());
        assert!(find(&host, "sapid-file-preview").is_empty());
        assert_eq!(files.stats().references, 0);
        assert_eq!(files.stats().preview_bytes, 0);
        assert!(
            control
                .read(
                    FileRead {
                        file,
                        offset: 0,
                        length: 1
                    },
                    |_| {}
                )
                .is_err()
        );
        host.app().clear().unwrap();
        assert!(host.app().tree().is_empty());
    }
}
#[test]
fn file_button_cancellation_and_refusal_preserve_the_current_controller_selection() {
    for theme in themes() {
        let picker = Arc::new(Picker::default());
        let files = Files::new(picker.clone());
        let host = WindowApp::with_config(Config {
            files: Some(files.clone()),
            ..Default::default()
        });
        let scope = host.app().scope();
        let controller = scope.cage(None);
        let selected = scope.cage(Vec::<FileInfo>::new());
        host.mount(
            sapid::file_button()
                .theme(theme)
                .controller_ref(controller)
                .files_ref(selected),
        )
        .unwrap();
        let control = controller.get().unwrap();
        *picker.0.lock().unwrap() = Some(Ok(Some(vec![SelectedFile {
            name: "note.txt".into(),
            media_type: "text/plain".into(),
            size: 1,
            reader: Arc::new(Reader(Arc::new(vec![7]))),
        }])));
        pick(&host);
        let previous = selected.get();
        assert_eq!(previous.len(), 1);
        for outcome in [
            Ok(None),
            Err(FileRejection::Busy),
            Err(FileRejection::Unavailable),
        ] {
            *picker.0.lock().unwrap() = Some(outcome);
            pick(&host);
            assert_eq!(selected.get(), previous);
            assert_eq!(files.stats().references, 1);
        }
        control.clear().unwrap();
        settle(&host);
        assert!(selected.get().is_empty());
        assert_eq!(files.stats().references, 0);
        host.app().clear().unwrap();
    }
}
