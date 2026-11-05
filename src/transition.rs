//! Owner-scoped presence with host presentation and revisioned completion.
use crate::appearance::Common;
use crate::composite::{self, Callback, ContentPolicy, Factory, Observers};
use glory::render::{Event, EventKind, NodeId, PresenceConfig, Semantics};
use glory::style::Style;
use glory::ui::{IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};

pub use glory::render::{
    PresenceEffect as TransitionEffect, PresenceStyle as TransitionStyles,
    PresenceTiming as TransitionTiming,
};

pub struct Transition {
    common: Common,
    mounted: Vary<bool>,
    effect: Vary<TransitionEffect>,
    duration: Vary<u16>,
    exit_duration: Option<Vary<u16>>,
    enter_delay: Vary<u16>,
    exit_delay: Vary<u16>,
    timing: Vary<TransitionTiming>,
    animated: Vary<bool>,
    distance: u16,
    appear: bool,
    policy: ContentPolicy,
    content: Option<Factory>,
    callbacks: [Option<Box<dyn FnMut()>>; 4],
}
pub fn transition() -> Transition {
    Transition {
        common: Common::default(),
        mounted: false.into(),
        effect: TransitionEffect::Fade.into(),
        duration: 200.into(),
        exit_duration: None,
        enter_delay: 0.into(),
        exit_delay: 0.into(),
        timing: TransitionTiming::Ease.into(),
        animated: true.into(),
        distance: 16,
        appear: false,
        policy: ContentPolicy::Unmount,
        content: None,
        callbacks: [None, None, None, None],
    }
}
impl Transition {
    pub fn mounted(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.mounted = value.into();
        self
    }
    pub fn effect(mut self, value: impl Into<Vary<TransitionEffect>>) -> Self {
        self.effect = value.into();
        self
    }
    pub fn duration(mut self, value: impl Into<Vary<u16>>) -> Self {
        self.duration = value.into();
        self
    }
    pub fn exit_duration(mut self, value: impl Into<Vary<u16>>) -> Self {
        self.exit_duration = Some(value.into());
        self
    }
    pub fn enter_delay(mut self, value: impl Into<Vary<u16>>) -> Self {
        self.enter_delay = value.into();
        self
    }
    pub fn exit_delay(mut self, value: impl Into<Vary<u16>>) -> Self {
        self.exit_delay = value.into();
        self
    }
    pub fn timing(mut self, value: impl Into<Vary<TransitionTiming>>) -> Self {
        self.timing = value.into();
        self
    }
    /// Explicit Reduce opt-out; the browser/native host may also apply its preference.
    pub fn animated(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.animated = value.into();
        self
    }
    pub fn distance(mut self, pixels: u16) -> Self {
        self.distance = pixels;
        self
    }
    pub fn appear(mut self, value: bool) -> Self {
        self.appear = value;
        self
    }
    pub fn content_policy(mut self, value: ContentPolicy) -> Self {
        self.policy = value;
        self
    }
    pub fn content<C: IntoChildren + 'static>(
        mut self,
        mut factory: impl FnMut() -> C + 'static,
    ) -> Self {
        self.content = Some(Box::new(move || {
            Box::new(glory::ui::group().fill(factory()))
        }));
        self
    }
    pub fn on_enter(mut self, callback: impl FnMut() + 'static) -> Self {
        self.callbacks[0] = Some(Box::new(callback));
        self
    }
    pub fn on_entered(mut self, callback: impl FnMut() + 'static) -> Self {
        self.callbacks[1] = Some(Box::new(callback));
        self
    }
    pub fn on_exit(mut self, callback: impl FnMut() + 'static) -> Self {
        self.callbacks[2] = Some(Box::new(callback));
        self
    }
    pub fn on_exited(mut self, callback: impl FnMut() + 'static) -> Self {
        self.callbacks[3] = Some(Box::new(callback));
        self
    }
    pub fn theme(mut self, value: crate::Theme) -> Self {
        self.common.theme = value;
        self
    }
    pub fn class(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.class = value.into();
        self
    }
    pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
        self.common.style = value.into();
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
    pub fn node_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.common.node_ref = Some(value);
        self
    }
    pub fn on(mut self, kind: EventKind, handler: impl FnMut(Event) + 'static) -> Self {
        self.common
            .handlers
            .entry(kind)
            .or_default()
            .push(Box::new(handler));
        self
    }
}
struct Source {
    mounted: Vary<bool>,
    effect: Vary<TransitionEffect>,
    duration: Vary<u16>,
    exit_duration: Vary<u16>,
    enter_delay: Vary<u16>,
    exit_delay: Vary<u16>,
    timing: Vary<TransitionTiming>,
    animated: Vary<bool>,
    distance: u16,
    appear: bool,
    completion: bool,
    transforms: bool,
}
impl Source {
    fn read(&self) -> Result<PresenceConfig> {
        // Validate both directions before publishing any accepted state/callback.
        let (duration, exit_duration, enter_delay, exit_delay) = (
            self.duration.try_get()?,
            self.exit_duration.try_get()?,
            self.enter_delay.try_get()?,
            self.exit_delay.try_get()?,
        );
        let open = self.mounted.try_get()?;
        let mut config = PresenceConfig {
            open,
            appear: self.appear,
            effect: self.effect.try_get()?,
            timing: self.timing.try_get()?,
            duration_millis: duration,
            delay_millis: enter_delay,
            distance: self.distance,
            revision: 0,
        };
        config.validate()?;
        PresenceConfig {
            duration_millis: exit_duration,
            delay_millis: exit_delay,
            ..config
        }
        .validate()?;
        if !open {
            config.duration_millis = exit_duration;
            config.delay_millis = exit_delay;
        }
        if !self.animated.try_get()? {
            config.duration_millis = 0;
            config.delay_millis = 0;
        }
        if self.completion
            && !self.transforms
            && config.duration_millis > 0
            && config.effect.style(config.distance).has_transform()
        {
            return Err(glory::render::Error::Unsupported(
                "native Transition supports fade/none; transform effects require the browser host",
            )
            .into());
        }
        Ok(config)
    }
}
struct State {
    source: Source,
    config: Cage<PresenceConfig>,
    presented: Cage<bool>,
    completed: Cell<Option<u64>>,
    policy: ContentPolicy,
    weak: glory::ui::WeakScope,
    callbacks: [Callback<()>; 4],
}
impl State {
    fn emit(&self, index: usize) {
        if let Some(scope) = self.weak.upgrade() {
            self.callbacks[index].emit(scope.id(), ());
        }
    }
    fn start(&self, config: PresenceConfig) -> Result {
        self.emit(if config.open { 0 } else { 2 });
        if self.weak.upgrade().is_none() {
            return Ok(());
        }
        if !self.source.completion {
            self.complete(config.revision)?;
        }
        Ok(())
    }
    fn complete(&self, revision: u64) -> Result {
        let config = self.config.try_get()?;
        let source = self.source.read()?;
        if config.revision != revision
            || self.completed.get() == Some(revision)
            || (PresenceConfig { revision, ..source }) != config
        {
            return Ok(());
        }
        self.completed.set(Some(revision));
        if !config.open && self.policy == ContentPolicy::Unmount {
            self.presented.try_revise(|v| *v = false)?;
        }
        self.emit(if config.open { 1 } else { 3 });
        Ok(())
    }
}
impl Widget for Transition {
    fn build(&mut self, scope: &Scope) -> Result {
        let capabilities = scope.capabilities()?;
        let source = Source {
            mounted: self.mounted.clone(),
            effect: self.effect.clone(),
            duration: self.duration.clone(),
            exit_duration: self
                .exit_duration
                .clone()
                .unwrap_or_else(|| self.duration.clone()),
            enter_delay: self.enter_delay.clone(),
            exit_delay: self.exit_delay.clone(),
            timing: self.timing.clone(),
            animated: self.animated.clone(),
            distance: self.distance,
            appear: self.appear,
            completion: capabilities.presence_completion,
            transforms: capabilities.presence_transforms,
        };
        let initial = source.read()?;
        let state = Rc::new(State {
            source,
            config: scope.cage(initial),
            presented: scope.cage(initial.open || self.policy == ContentPolicy::KeepMounted),
            completed: Cell::new(if initial.open && self.appear {
                None
            } else {
                Some(0)
            }),
            policy: self.policy,
            weak: scope.downgrade(),
            callbacks: std::mem::take(&mut self.callbacks).map(|callback| {
                Callback::new(
                    callback
                        .map(|mut callback| Box::new(move |()| callback()) as Box<dyn FnMut(())>),
                )
            }),
        });
        let completion = state.clone();
        let config = state.config;
        let (classes, style) = self
            .common
            .appearance(scope, "sapid-transition", Style::new());
        let observers = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::PresenceComplete)
                .unwrap_or_default(),
        );
        let observed = observers.clone();
        let weak = scope.downgrade();
        let mut shell = glory::ui::view()
            .class(classes)
            .style(style)
            .semantics(scope.bond(move || {
                let config = config.get();
                composite::metadata(Semantics {
                    hidden: !config.open,
                    inert: !config.open,
                    presence: Some(Box::new(config)),
                    ..Default::default()
                })
            }))
            .on(EventKind::PresenceComplete, move |event| {
                if let Event::PresenceComplete { revision } = event {
                    if let Err(error) = completion.complete(revision) {
                        if let Some(scope) = weak.upgrade() {
                            scope.report_error(error);
                        }
                        return;
                    }
                    if let Some(scope) = weak.upgrade() {
                        observed.emit(scope.id(), event);
                    }
                }
            });
        if let Some(reference) = self.common.node_ref {
            shell = shell.node_ref(reference);
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            shell = shell.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        if let Some(factory) = self.content.take() {
            shell = shell.fill(glory::ui::when(state.presented, factory));
        }
        shell.show_in(scope)?;
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = cleanup.upgrade() {
                for callback in &state.callbacks {
                    callback.clear();
                }
            }
            observers.clear();
        })?;
        let binding = state.clone();
        scope.structural(move || {
            let Some(scope) = binding.weak.upgrade() else {
                return;
            };
            let result = (|| -> Result {
                let mut next = binding.source.read()?;
                let previous = glory::core::untracked(|| binding.config.try_get())?;
                next.revision = previous.revision;
                if next == previous {
                    return Ok(());
                }
                next.revision = previous
                    .revision
                    .checked_add(1)
                    .ok_or(glory::render::Error::PresenceRevisionExhausted)?;
                if next.open || binding.policy == ContentPolicy::KeepMounted {
                    binding.presented.try_revise(|v| *v = true)?;
                }
                binding.config.try_revise(|v| *v = next)?;
                binding.start(next)
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        if initial.open && self.appear {
            state.start(initial)?;
        }
        Ok(())
    }
}
