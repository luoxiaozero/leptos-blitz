use leptos::tachys::{
    html::attribute::{Attribute, NextAttribute},
    renderer::{RemoveEventHandler, Renderer},
};
use send_wrapper::SendWrapper;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use std::{borrow::Cow, cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

thread_local! {
    static EVENTS: RefCell<SlotMap<DefaultKey, Box<dyn FnMut(Event)>>> = Default::default();
}

pub type SharedEventCallback<E> = Rc<RefCell<dyn FnMut(E)>>;

pub trait EventCallback<E>: 'static {
    fn invoke(&mut self, event: E);

    fn into_shared(self) -> SharedEventCallback<E>;
}

impl<E: 'static> EventCallback<E> for SharedEventCallback<E> {
    fn invoke(&mut self, event: E) {
        let mut fun = self.borrow_mut();
        fun(event)
    }

    fn into_shared(self) -> SharedEventCallback<E> {
        self
    }
}

impl<F, E> EventCallback<E> for F
where
    F: FnMut(E) + 'static,
{
    fn invoke(&mut self, event: E) {
        self(event)
    }

    fn into_shared(self) -> SharedEventCallback<E> {
        Rc::new(RefCell::new(self))
    }
}

pub fn on<E, R, F>(event: E, cb: F) -> On<E, F, R>
where
    F: FnMut(E::EventType) + 'static,
    E: EventDescriptor + Send + 'static,
    E::EventType: 'static,
    R: Renderer,
    E::EventType: From<Event>,
{
    On {
        event,
        cb: SendWrapper::new(cb),
        ty: PhantomData,
    }
}

pub struct On<E, F, R> {
    event: E,
    cb: SendWrapper<F>,
    ty: PhantomData<R>,
}

impl<E, F, R> Clone for On<E, F, R>
where
    E: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
            cb: self.cb.clone(),
            ty: PhantomData,
        }
    }
}

impl<E, F, R> On<E, F, R>
where
    F: EventCallback<E::EventType>,
    E: EventDescriptor + Send + 'static,
    E::EventType: 'static,
    R: Renderer,
    E::EventType: From<Event>,
{
    pub fn attach(self, el: &R::Element) -> RemoveEventHandler<R::Element> {
        fn attach_inner<R: Renderer>(
            el: &R::Element,
            cb: Box<dyn FnMut(Event)>,
            html_name: Cow<'static, str>,
            _delegation_key: Option<Cow<'static, str>>,
        ) -> RemoveEventHandler<R::Element> {
            let key = Event::insert(cb);
            R::set_attribute(el, &html_name, &key.to_string());

            Box::new({
                move |_el| {
                    Event::remove(key);
                }
            })
        }

        let mut cb = self.cb.take();

        let cb = Box::new(move |ev: Event| {
            let ev = E::EventType::from(ev);
            cb.invoke(ev);
        }) as Box<dyn FnMut(Event)>;

        attach_inner::<R>(
            el,
            cb,
            self.event.html_name(),
            (E::BUBBLES).then(|| self.event.event_delegation_key()),
        )
    }
}

impl<E, F, R> Debug for On<E, F, R>
where
    E: Debug,
    R: Renderer,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("On").field(&self.event).finish()
    }
}

impl<E, F, R> Attribute<R> for On<E, F, R>
where
    F: EventCallback<E::EventType>,
    E: EventDescriptor + Send + 'static,
    E::EventType: 'static,
    R: Renderer,
    E::EventType: From<Event>,
{
    const MIN_LENGTH: usize = 0;
    // a function that can be called once to remove the event listener
    type State = (R::Element, Option<Box<dyn FnOnce(&R::Element)>>);
    type Cloneable = On<E, SharedEventCallback<E::EventType>, R>;
    type CloneableOwned = On<E, SharedEventCallback<E::EventType>, R>;

    #[inline(always)]
    fn html_len(&self) -> usize {
        0
    }

    #[inline(always)]
    fn to_html(
        self,
        _buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
    ) {
    }

    #[inline(always)]
    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        let cleanup = self.attach(el);
        (el.clone(), Some(cleanup))
    }

    #[inline(always)]
    fn build(self, el: &R::Element) -> Self::State {
        let cleanup = self.attach(el);
        (el.clone(), Some(cleanup))
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (el, prev_cleanup) = state;
        if let Some(prev) = prev_cleanup.take() {
            prev(el);
        }
        *prev_cleanup = Some(self.attach(el));
    }

    fn into_cloneable(self) -> Self::Cloneable {
        On {
            cb: SendWrapper::new(self.cb.take().into_shared()),
            event: self.event,
            ty: self.ty,
        }
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        On {
            cb: SendWrapper::new(self.cb.take().into_shared()),
            event: self.event,
            ty: self.ty,
        }
    }
}

impl<E, F, R> NextAttribute<R> for On<E, F, R>
where
    F: EventCallback<E::EventType>,
    E: EventDescriptor + Send + 'static,
    E::EventType: 'static,
    R: Renderer,
    E::EventType: From<Event>,
{
    type Output<NewAttr: Attribute<R>> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute<R>>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// A trait for converting types into [web_sys events](web_sys).
pub trait EventDescriptor: Clone {
    /// The [`web_sys`] event type, such as [`web_sys::MouseEvent`].
    // type EventType: FromWasmAbi;
    type EventType;

    /// Indicates if this event bubbles. For example, `click` bubbles,
    /// but `focus` does not.
    ///
    /// If this is true, then the event will be delegated globally,
    /// otherwise, event listeners will be directly attached to the element.
    const BUBBLES: bool;

    /// The name of the event, such as `click` or `mouseover`.
    fn name(&self) -> Cow<'static, str>;

    /// e.g. `onclick` or `onmouseover`
    fn html_name(&self) -> Cow<'static, str>;

    /// The key used for event delegation.
    fn event_delegation_key(&self) -> Cow<'static, str>;

    // /// Return the options for this type. This is only used when you create a [`Custom`] event
    // /// handler.
    // #[inline(always)]
    // fn options(&self) -> &Option<web_sys::AddEventListenerOptions> {
    //     &None
    // }
}

pub struct Event;

impl Event {
    pub(crate) fn insert(cb: Box<dyn FnMut(Event)>) -> u64 {
        let key = EVENTS.with_borrow_mut(|events| events.insert(cb));

        key.data().as_ffi()
    }

    pub(crate) fn remove(key: u64) {
        let key = KeyData::from_ffi(key).into();

        EVENTS.with_borrow_mut(|events| {
            events.remove(key);
        });
    }

    pub(crate) fn call_mut(key: u64) {
        let key = KeyData::from_ffi(key).into();

        EVENTS.with_borrow_mut(|events| {
            if let Some(event) = events.get_mut(key) {
                event(Event {});
            }
        });
    }
}

macro_rules! generate_event_types {
  {$(
    $( #[$does_not_bubble:ident] )?
    $( $event:ident )+ : $web_event:ident
  ),* $(,)?} => {
    ::paste::paste! {
      $(
        #[doc = "The `" [< $($event)+ >] "` event, which receives [" $web_event "](web_sys::" $web_event ") as its argument."]
        #[derive(Copy, Clone, Debug)]
        #[allow(non_camel_case_types)]
        pub struct [<$( $event )+ >];

        impl EventDescriptor for [< $($event)+ >] {
          type EventType = Event;

          #[inline(always)]
          fn name(&self) -> Cow<'static, str> {
            stringify!([< $($event)+ >]).into()
          }

          #[inline(always)]
          fn html_name(&self) -> Cow<'static, str> {
            concat!("on", stringify!([< $($event)+ >])).into()
          }

          #[inline(always)]
          fn event_delegation_key(&self) -> Cow<'static, str> {
            concat!("$$$", stringify!([< $($event)+ >])).into()
          }

          const BUBBLES: bool = true $(&& generate_event_types!($does_not_bubble))?;
        }
      )*
    }
  };

  (does_not_bubble) => { false }
}

generate_event_types! {
  // =========================================================
  // WindowEventHandlersEventMap
  // =========================================================
  #[does_not_bubble]
  after print: Event,
  #[does_not_bubble]
  before print: Event,
  #[does_not_bubble]
  before unload: BeforeUnloadEvent,
  #[does_not_bubble]
  gamepad connected: GamepadEvent,
  #[does_not_bubble]
  gamepad disconnected: GamepadEvent,
  hash change: HashChangeEvent,
  #[does_not_bubble]
  language change: Event,
  #[does_not_bubble]
  message: MessageEvent,
  #[does_not_bubble]
  message error: MessageEvent,
  #[does_not_bubble]
  offline: Event,
  #[does_not_bubble]
  online: Event,
  #[does_not_bubble]
  page hide: PageTransitionEvent,
  #[does_not_bubble]
  page show: PageTransitionEvent,
  pop state: PopStateEvent,
  rejection handled: PromiseRejectionEvent,
  #[does_not_bubble]
  storage: StorageEvent,
  #[does_not_bubble]
  unhandled rejection: PromiseRejectionEvent,
  #[does_not_bubble]
  unload: Event,

  // =========================================================
  // GlobalEventHandlersEventMap
  // =========================================================
  #[does_not_bubble]
  abort: UiEvent,
  animation cancel: AnimationEvent,
  animation end: AnimationEvent,
  animation iteration: AnimationEvent,
  animation start: AnimationEvent,
  aux click: MouseEvent,
  before input: InputEvent,
  before toggle: Event, // web_sys does not include `ToggleEvent`
  #[does_not_bubble]
  blur: FocusEvent,
  #[does_not_bubble]
  can play: Event,
  #[does_not_bubble]
  can play through: Event,
  change: Event,
  click: MouseEvent,
  #[does_not_bubble]
  close: Event,
  composition end: CompositionEvent,
  composition start: CompositionEvent,
  composition update: CompositionEvent,
  context menu: MouseEvent,
  #[does_not_bubble]
  cue change: Event,
  dbl click: MouseEvent,
  drag: DragEvent,
  drag end: DragEvent,
  drag enter: DragEvent,
  drag leave: DragEvent,
  drag over: DragEvent,
  drag start: DragEvent,
  drop: DragEvent,
  #[does_not_bubble]
  duration change: Event,
  #[does_not_bubble]
  emptied: Event,
  #[does_not_bubble]
  ended: Event,
  #[does_not_bubble]
  error: ErrorEvent,
  #[does_not_bubble]
  focus: FocusEvent,
  #[does_not_bubble]
  focus in: FocusEvent,
  #[does_not_bubble]
  focus out: FocusEvent,
  form data: Event, // web_sys does not include `FormDataEvent`
  #[does_not_bubble]
  got pointer capture: PointerEvent,
  input: Event,
  #[does_not_bubble]
  invalid: Event,
  key down: KeyboardEvent,
  key press: KeyboardEvent,
  key up: KeyboardEvent,
  #[does_not_bubble]
  load: Event,
  #[does_not_bubble]
  loaded data: Event,
  #[does_not_bubble]
  loaded metadata: Event,
  #[does_not_bubble]
  load start: Event,
  lost pointer capture: PointerEvent,
  mouse down: MouseEvent,
  #[does_not_bubble]
  mouse enter: MouseEvent,
  #[does_not_bubble]
  mouse leave: MouseEvent,
  mouse move: MouseEvent,
  mouse out: MouseEvent,
  mouse over: MouseEvent,
  mouse up: MouseEvent,
  #[does_not_bubble]
  pause: Event,
  #[does_not_bubble]
  play: Event,
  #[does_not_bubble]
  playing: Event,
  pointer cancel: PointerEvent,
  pointer down: PointerEvent,
  #[does_not_bubble]
  pointer enter: PointerEvent,
  #[does_not_bubble]
  pointer leave: PointerEvent,
  pointer move: PointerEvent,
  pointer out: PointerEvent,
  pointer over: PointerEvent,
  pointer up: PointerEvent,
  #[does_not_bubble]
  progress: ProgressEvent,
  #[does_not_bubble]
  rate change: Event,
  reset: Event,
  #[does_not_bubble]
  resize: UiEvent,
  #[does_not_bubble]
  scroll: Event,
  #[does_not_bubble]
  scroll end: Event,
  security policy violation: SecurityPolicyViolationEvent,
  #[does_not_bubble]
  seeked: Event,
  #[does_not_bubble]
  seeking: Event,
  select: Event,
  #[does_not_bubble]
  selection change: Event,
  select start: Event,
  slot change: Event,
  #[does_not_bubble]
  stalled: Event,
  submit: SubmitEvent,
  #[does_not_bubble]
  suspend: Event,
  #[does_not_bubble]
  time update: Event,
  #[does_not_bubble]
  toggle: Event,
  touch cancel: TouchEvent,
  touch end: TouchEvent,
  touch move: TouchEvent,
  touch start: TouchEvent,
  transition cancel: TransitionEvent,
  transition end: TransitionEvent,
  transition run: TransitionEvent,
  transition start: TransitionEvent,
  #[does_not_bubble]
  volume change: Event,
  #[does_not_bubble]
  waiting: Event,
  webkit animation end: Event,
  webkit animation iteration: Event,
  webkit animation start: Event,
  webkit transition end: Event,
  wheel: WheelEvent,

  // =========================================================
  // WindowEventMap
  // =========================================================
  D O M Content Loaded: Event, // Hack for correct casing
  #[does_not_bubble]
  device motion: DeviceMotionEvent,
  #[does_not_bubble]
  device orientation: DeviceOrientationEvent,
  #[does_not_bubble]
  orientation change: Event,

  // =========================================================
  // DocumentAndElementEventHandlersEventMap
  // =========================================================
  copy: Event, // ClipboardEvent is unstable
  cut: Event, // ClipboardEvent is unstable
  paste: Event, // ClipboardEvent is unstable

  // =========================================================
  // DocumentEventMap
  // =========================================================
  fullscreen change: Event,
  fullscreen error: Event,
  pointer lock change: Event,
  pointer lock error: Event,
  #[does_not_bubble]
  ready state change: Event,
  visibility change: Event,
}

// Export `web_sys` event types
// use super::{attribute::NextAttribute, element::HasElementType};
// #[doc(no_inline)]
// pub use web_sys::{
//     AnimationEvent, BeforeUnloadEvent, CompositionEvent, CustomEvent,
//     DeviceMotionEvent, DeviceOrientationEvent, DragEvent, ErrorEvent, Event,
//     FocusEvent, GamepadEvent, HashChangeEvent, InputEvent, KeyboardEvent,
//     MessageEvent, MouseEvent, PageTransitionEvent, PointerEvent, PopStateEvent,
//     ProgressEvent, PromiseRejectionEvent, SecurityPolicyViolationEvent,
//     StorageEvent, SubmitEvent, TouchEvent, TransitionEvent, UiEvent,
//     WheelEvent,
// };
