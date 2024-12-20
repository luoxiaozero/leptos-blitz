use super::event_listener::EventListener;

struct EventTarget {}

impl EventTarget {
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    pub fn add_event_listener_with_callback<T>(&self, r#type: &str, listener: impl Fn(T)) {}

    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    pub fn add_event_listener_with_event_listener<T>(
        &self,
        r#type: &str,
        listener: impl EventListener<T>,
    ) {
    }
}
