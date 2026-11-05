#[path = "../src/carousel/model.rs"]
mod model;
use glory::render::Direction;
use model::*;
#[test]
fn indicators_stay_bounded_and_cover_current_at_both_ends_and_the_middle() {
    for count in [0, 1, 3, 9, 10, MAX_CAROUSEL_COUNT] {
        for current in [0, count / 2, count.saturating_sub(1)] {
            let page = Page {
                count,
                current,
                looping: false,
                overscan: 1,
            };
            let indicators = page.indicators();
            assert_eq!(indicators.len(), count.min(MAX_CAROUSEL_INDICATORS));
            assert!(indicators.windows(2).all(|pair| pair[1] == pair[0] + 1));
            assert!(indicators.iter().all(|index| *index < count));
            if count > 0 {
                assert!(indicators.contains(&current));
            }
        }
    }
}
#[test]
fn million_slide_window_is_bounded_and_edges_do_not_wrap_implicitly() {
    let page = Page {
        count: MAX_CAROUSEL_COUNT,
        current: 500_000,
        overscan: 3,
        looping: false,
    }
    .validate()
    .unwrap();
    assert_eq!(page.window(), (499_997..=500_003).collect::<Vec<_>>());
    assert_eq!(page.propose(Step::Next), Some(500_001));
    assert_eq!(page.propose(Step::Previous), Some(499_999));
    assert_eq!(page.propose(Step::First), Some(0));
    assert_eq!(page.propose(Step::Last), Some(999_999));
    let first = Page { current: 0, ..page };
    assert_eq!(first.propose(Step::Previous), None);
    assert_eq!(first.window(), vec![0, 1, 2, 3]);
    let looping = Page {
        looping: true,
        ..first
    };
    assert_eq!(looping.propose(Step::Previous), Some(999_999));
    assert_eq!(
        looping.window(),
        vec![0, 1, 2, 3, 999_997, 999_998, 999_999]
    );
}
#[test]
fn empty_single_and_invalid_sources_are_explicit() {
    for count in [0, 1] {
        let page = Page {
            count,
            current: 0,
            overscan: 3,
            looping: true,
        }
        .validate()
        .unwrap();
        assert_eq!(page.propose(Step::Next), None);
        assert_eq!(page.window(), if count == 0 { vec![] } else { vec![0] });
    }
    for page in [
        Page {
            count: 0,
            current: 1,
            overscan: 1,
            looping: false,
        },
        Page {
            count: 2,
            current: 2,
            overscan: 1,
            looping: false,
        },
        Page {
            count: MAX_CAROUSEL_COUNT + 1,
            current: 0,
            overscan: 1,
            looping: false,
        },
        Page {
            count: 2,
            current: 0,
            overscan: 4,
            looping: false,
        },
    ] {
        assert!(page.validate().is_err());
    }
}
#[test]
fn swipe_threshold_direction_and_finite_geometry() {
    let page = Page {
        count: 10,
        current: 5,
        overscan: 1,
        looping: false,
    };
    assert_eq!(page.swipe(-40., 300., Direction::Ltr).unwrap(), None);
    assert_eq!(page.swipe(-60., 300., Direction::Ltr).unwrap(), Some(6));
    assert_eq!(page.swipe(-60., 300., Direction::Rtl).unwrap(), Some(4));
    assert_eq!(page.swipe(30., 10., Direction::Ltr).unwrap(), Some(4));
    for (delta, extent) in [
        (f64::NAN, 100.),
        (f64::INFINITY, 100.),
        (100., 0.),
        (100., f64::INFINITY),
    ] {
        assert!(page.swipe(delta, extent, Direction::Ltr).is_err());
    }
}
